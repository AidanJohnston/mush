use std::{
    io::{BufRead, BufReader, Error, ErrorKind, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use super::diagnostics::error::MushError;

const BUFFER_SIZE: usize = 4;

#[derive(Debug)]
pub enum KeywordToken {
    // Single Character
    LeftParen,
    RightParen,
    LeftCurl,
    RightCurl,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    NewLine,
    Slash,
    Star,

    // Comparison
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords
    And,
    Fn,
    For,
    If,
    None,
    Or,
    Return,
    True,
    False,
    Let,
    While,

    EndOfFile,
}

impl KeywordToken {
    fn as_str(&self) -> &str {
        match self {
            KeywordToken::LeftParen => "(",
            KeywordToken::RightParen => ")",
            KeywordToken::LeftCurl => todo!(),
            KeywordToken::RightCurl => todo!(),
            KeywordToken::Comma => todo!(),
            KeywordToken::Dot => todo!(),
            KeywordToken::Minus => todo!(),
            KeywordToken::Plus => todo!(),
            KeywordToken::SemiColon => todo!(),
            KeywordToken::NewLine => todo!(),
            KeywordToken::Slash => todo!(),
            KeywordToken::Star => todo!(),
            KeywordToken::Bang => todo!(),
            KeywordToken::BangEqual => todo!(),
            KeywordToken::Equal => todo!(),
            KeywordToken::EqualEqual => todo!(),
            KeywordToken::Greater => todo!(),
            KeywordToken::GreaterEqual => todo!(),
            KeywordToken::Less => todo!(),
            KeywordToken::LessEqual => todo!(),
            KeywordToken::And => todo!(),
            KeywordToken::Fn => todo!(),
            KeywordToken::For => todo!(),
            KeywordToken::If => todo!(),
            KeywordToken::None => todo!(),
            KeywordToken::Or => todo!(),
            KeywordToken::Return => todo!(),
            KeywordToken::True => todo!(),
            KeywordToken::False => todo!(),
            KeywordToken::Let => todo!(),
            KeywordToken::While => todo!(),
            KeywordToken::EndOfFile => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum LexemeToken {
    Identifier(String),
    String(String),
    Interger(String),
    Float(String),
}

pub enum Token {
    KeywordToken { token_type: KeywordToken },
    LexemeToken { token_type: LexemeToken },
}

#[derive(Clone)]
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

        self.add_keyword_token(KeywordToken::EndOfFile);

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let (c, new_buffer_position) = self.advance(self.buf_reader_position)?;
        self.buf_reader_position = new_buffer_position;
        self.scanner_ctx.increment_offset();

        match c {
            '(' => self.add_keyword_token(KeywordToken::LeftParen),
            ')' => self.add_keyword_token(KeywordToken::RightParen),
            '{' => self.add_keyword_token(KeywordToken::LeftCurl),
            '}' => self.add_keyword_token(KeywordToken::RightCurl),
            ',' => self.add_keyword_token(KeywordToken::Comma),
            '.' => self.add_keyword_token(KeywordToken::Dot),
            '-' => self.add_keyword_token(KeywordToken::Minus),
            '+' => self.add_keyword_token(KeywordToken::Plus),
            ';' => self.add_keyword_token(KeywordToken::SemiColon),
            '*' => self.add_keyword_token(KeywordToken::Star),
            '!' => self.match_bang()?,
            '/' => self.match_slash()?,
            '"' => self.match_string()?,
            '1'..'9' => self.match_number()?,
            ' ' | '\r' | '\t' => { /* Do nothing, skip the spaces (we didn't need them anyway) */ }
            '\n' => {
                self.add_keyword_token(KeywordToken::NewLine);
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

    fn match_bang(&mut self) -> Result<(), Error> {
        if self.is_done()? {
            return Ok(());
        }

        let (next, new_buffer_position) = self.advance(self.buf_reader_position)?;
        match next {
            '=' => {
                self.add_keyword_token(KeywordToken::BangEqual);
                self.buf_reader_position = new_buffer_position;
                self.scanner_ctx.increment_offset();
            }
            _ => self.add_keyword_token(KeywordToken::Bang),
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
            _ => self.add_keyword_token(KeywordToken::Slash),
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
            string.push(next);
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

    fn add_keyword_token(&mut self, token_type: KeywordToken) {
        let token = Token::KeywordToken { token_type };
        self.tokens.push(token)
    }

    fn add_lexeme_token(&mut self, token_type: LexemeToken) {
        let token = Token::LexemeToken { token_type };
        self.tokens.push(token)
    }

    pub fn has_errors(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn errors(&self) -> &Vec<MushError> {
        &self.errors
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}
