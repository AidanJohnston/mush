use std::{
    io::{BufRead, BufReader, Error, ErrorKind, Read, Seek, SeekFrom},
    path::PathBuf,
};

use super::{
    diagnostics::error::MushError,
    tokens::{SingleToken, Token},
};

const BUFFER_SIZE: usize = 4;

#[derive(Debug, Clone)]
pub struct MushContext {
    file: PathBuf,
    line: String,
    line_number: u64,
    offset: u64,
}
impl MushContext {
    pub fn new(file: PathBuf, line: String, line_number: u64, offset: u64) -> Self {
        Self {
            file,
            line,
            line_number,
            offset,
        }
    }

    pub fn line(&self) -> &str {
        &self.line
    }

    pub fn set_line(&mut self, line: String) {
        self.line = line;
    }

    pub fn line_number(&self) -> u64 {
        self.line_number
    }

    pub fn set_line_number(&mut self, line_number: u64) {
        self.line_number = line_number;
    }

    pub fn increment_line_number(&mut self) {
        self.line_number += 1;
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }

    pub fn increment_offset(&mut self) {
        self.offset += 1
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }
}

pub struct Scanner<R>
where
    R: Seek + Read,
{
    buf_reader: BufReader<R>,
    buf_reader_position: u64,
    tokens: Vec<Token>,
    errors: Vec<MushError>,
    scanner_ctx: MushContext,
}
impl<R> Scanner<R>
where
    R: Seek + Read,
{
    pub fn new(buf_reader: BufReader<R>, scanner_ctx: MushContext) -> Self {
        Self {
            buf_reader,
            buf_reader_position: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
            scanner_ctx,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), Error> {
        self.buf_reader.rewind()?;

        let mut is_done = self.is_done()?;
        while !is_done {
            self.scan_token()?;
            is_done = self.is_done()?;
        }

        self.add_keyword_token(Token::EndOfFile);

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let (c, new_buffer_position) = self.advance(self.buf_reader_position)?;
        self.buf_reader_position = new_buffer_position;
        self.scanner_ctx.increment_offset();

        match c {
            '(' => self.add_single_token(SingleToken::LeftParen),
            ')' => self.add_single_token(SingleToken::RightParen),
            '{' => self.add_single_token(SingleToken::LeftCurl),
            '}' => self.add_single_token(SingleToken::RightCurl),
            ',' => self.add_single_token(SingleToken::Comma),
            '.' => self.add_single_token(SingleToken::Dot),
            '-' => self.add_single_token(SingleToken::Minus),
            '+' => self.add_single_token(SingleToken::Plus),
            ';' => self.add_single_token(SingleToken::SemiColon),
            '*' => self.add_single_token(SingleToken::Star),
            '!' => self.match_next()?,
            '=' => self.match_equal()?,
            // '>' -> self.match_greater()?,
            // '<' -> self.match_less()?,
            '/' => self.match_slash()?,
            '"' => self.match_string()?,
            '1'..'9' => self.match_number()?,
            ' ' | '\r' | '\t' => { /* Do nothing, skip the spaces (we didn't need them anyway) */ }
            '\n' => {
                self.add_keyword_token(Token::NewLine);
                self.scanner_ctx.increment_line_number();
                self.scanner_ctx.increment_offset();
            }
            _ => self.add_error(MushError::UnknownCharacter {
                unknown_character: c,
                ctx: self.scanner_ctx.clone(),
            }),
        };
        Ok(())
    }

    fn advance(&mut self, buffer_position: u64) -> Result<(char, u64), Error> {
        self.buf_reader.seek(SeekFrom::Start(buffer_position))?;

        let mut buffer = [0; BUFFER_SIZE];

        for i in 0..BUFFER_SIZE {
            self.buf_reader.read_exact(&mut buffer[i..i + 1])?;
            if let Ok(str_slice) = std::str::from_utf8(&buffer[..=i]) {
                if let Some(character) = str_slice.chars().next() {
                    let new_buffer_position = buffer_position + 1 + i as u64;
                    return Ok((character, new_buffer_position));
                }
            }
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode a UTF-8 characters",
        ))
    }

    fn match_next(&mut self, single_token: Token, composite_token: Token) -> Result<(), Error> {
        if self.is_done()? {
            return Ok(());
        }
        let (next, new_buffer_position) = self.advance(self.buf_reader_position)?;
        if next == next_token_char {
            self.add_keyword_token(composite_token);
            self.buf_reader_position = new_buffer_position;
            self.scanner_ctx.increment_offset();
            return Ok(());
        }
        self.add_keyword_token(single_token);
        Ok(())
    }

    fn match_equal(&mut self) -> Result<(), Error> {
        if self.is_done()? {
            return Ok(());
        }

        let (next, new_buffer_position) = self.advance(self.buf_reader_position)?;
        match next {
            '=' => {
                self.add_keyword_token(Token::EqualEqual);
                self.buf_reader_position = new_buffer_position;
                self.scanner_ctx.increment_offset();
            }
            _ => self.add_keyword_token(Token::Equal),
        }
        Ok(())
    }

    fn match_slash(&mut self) -> Result<(), Error> {
        if self.is_done()? {
            return Ok(());
        }

        let (next, mut new_buffer_position) = self.advance(self.buf_reader_position)?;
        match next {
            '/' => {
                // Comsume the rest of the commet
                loop {
                    if self.is_done()? {
                        return Ok(());
                    }
                    let (next, next_buffer_position) = self.advance(new_buffer_position)?;
                    new_buffer_position = next_buffer_position;
                    if next == '\n' {
                        break;
                    }
                }
            }
            _ => self.add_keyword_token(Token::Slash),
        }
        return Ok(());
    }

    fn match_string(&mut self) -> Result<(), Error> {
        if self.is_done()? {
            return Ok(());
        }

        let mut string = String::new();
        let mut new_buffer_position = self.buf_reader_position;
        loop {
            if self.is_done()? {
                return Ok(());
            }
            let (next, next_buffer_position) = self.advance(new_buffer_position)?;
            new_buffer_position = next_buffer_position;
            match next {
                '\n' => {
                    self.add_error(MushError::IncompleteString {
                        ctx: self.scanner_ctx.clone(),
                    });
                    return Ok(());
                }
                '"' => {
                    self.add_lexeme_token(LexemeToken::String(string));
                    return Ok(());
                }
                _ => string.push(next),
            }
        }
    }

    fn match_number(&mut self) -> Result<(), Error> {
        if self.is_done()? {
            return Ok(());
        }
        let mut number = String::new();
        let mut new_buffer_position = self.buf_reader_position;
        loop {
            if self.is_done()? {
                return Ok(());
            }
            let (next, next_buffer_position) = self.advance(new_buffer_position)?;
            new_buffer_position = next_buffer_position;
            number.push(next);
            match next {
                '0'..'9' => number.push(next),
                '.' => {
                    number.push('.');
                    loop {
                        let (next, next_buffer_position) = self.advance(new_buffer_position)?;
                        new_buffer_position = next_buffer_position;
                        match next {
                            '0'..'9' => number.push(next),
                            _ => {
                                self.add_lexeme_token(LexemeToken::Float(number));
                                return Ok(());
                            }
                        }
                    }
                }
                _ => {
                    self.add_lexeme_token(LexemeToken::Interger(number));
                    return Ok(());
                }
            }
        }
    }

    fn is_done(&mut self) -> Result<bool, Error> {
        let buffer = self.buf_reader.fill_buf()?;
        Ok(buffer.is_empty())
    }

    fn add_error(&mut self, mush_error: MushError) {
        self.errors.push(mush_error)
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token)
    }

    fn add_single_token(&mut self, single_token: SingleToken) {
        let token = Token::SingleToken(single_token);
        self.tokens.push(token)
    }

    pub fn has_errors(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn errors(&self) -> &[MushError] {
        &self.errors
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}
