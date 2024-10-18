use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Error, ErrorKind, Read, Seek, SeekFrom},
};

use super::diagnostics::error::{IncompleteString, MushError, UnknownCharacter};

const BUFFER_SIZE: usize = 4;

pub enum TokenType {
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

    Identifier,
    String,
    Interger,
    Float,

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

fn get_token_from_keyword(keyword: &str) -> Option<TokenType> {
    match keyword {
        "and" => Some(TokenType::And),
        "fn" => Some(TokenType::Fn),
        "for" => Some(TokenType::For),
        "if" => Some(TokenType::If),
        "None" => Some(TokenType::None),
        "or" => Some(TokenType::Or),
        "return" => Some(TokenType::Return),
        "True" => Some(TokenType::True),
        "False" => Some(TokenType::False),
        "let" => Some(TokenType::Let),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

trait Token {
    fn token_type(&self) -> &TokenType;
    fn line(&self) -> i32;
    fn offset(&self) -> i32;
}

pub struct LexemeToken {
    token_type: TokenType,
    lexeme: String,
    line: i32,
    offset: i32,
}
impl LexemeToken {
    pub fn new(token_type: TokenType, lexeme: &str, line: i32, offset: i32) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_owned(),
            line,
            offset,
        }
    }

    fn lexeme(&self) -> &str {
        self.lexeme.as_ref()
    }
}
impl Token for LexemeToken {
    fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    fn line(&self) -> i32 {
        self.line
    }

    fn offset(&self) -> i32 {
        self.offset
    }
}

pub struct KeywordToken {
    token_type: TokenType,
    line: i32,
    offset: i32,
}
impl KeywordToken {
    pub fn new(token_type: TokenType, line: i32, offset: i32) -> Self {
        Self {
            token_type,
            line,
            offset,
        }
    }
}
impl Token for KeywordToken {
    fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    fn line(&self) -> i32 {
        self.line
    }

    fn offset(&self) -> i32 {
        self.offset
    }
}

pub struct Scanner<R>
where
    R: Seek + Read,
{
    buf_reader: BufReader<R>,
    tokens: Vec<Box<dyn Token>>,
    errors: Vec<MushError>,

    current_line: i32,
    offset: i32,
    buf_reader_position: u64,
}
impl<R> Scanner<R>
where
    R: Seek + Read,
{
    pub fn new(buf_reader: BufReader<R>) -> Self {
        Self {
            buf_reader,
            tokens: Vec::new(),
            errors: Vec::new(),
            current_line: 1,
            offset: 0,
            buf_reader_position: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), Error> {
        self.buf_reader.rewind()?;

        let mut is_done = self.is_done()?;
        while !is_done {
            self.scan_token()?;
            is_done = self.is_done()?;
        }

        self.add_keyword_token(TokenType::EndOfFile);

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let (c, new_buffer_position) = self.advance(self.buf_reader_position)?;
        self.buf_reader_position = new_buffer_position;
        self.offset += 1;

        match c {
            '(' => self.add_keyword_token(TokenType::LeftParen),
            ')' => self.add_keyword_token(TokenType::RightParen),
            '{' => self.add_keyword_token(TokenType::LeftCurl),
            '}' => self.add_keyword_token(TokenType::RightCurl),
            ',' => self.add_keyword_token(TokenType::Comma),
            '.' => self.add_keyword_token(TokenType::Dot),
            '-' => self.add_keyword_token(TokenType::Minus),
            '+' => self.add_keyword_token(TokenType::Plus),
            ';' => self.add_keyword_token(TokenType::SemiColon),
            '*' => self.add_keyword_token(TokenType::Star),
            '!' => self.match_bang()?,
            '/' => self.match_slash()?,
            '"' => self.match_string()?,
            '1'..'9' => self.match_number()?,
            ' ' | '\r' | '\t' => { /* Do nothing, skip the spaces (we didn't need them anyway) */ }
            '\n' => {
                self.add_keyword_token(TokenType::NewLine);
                self.current_line += 1;
                self.offset = 0;
            }
            _ => self.add_error(MushError::UnknownCharacter(UnknownCharacter::new(
                c,
                self.current_line,
                self.offset,
            ))),
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
                self.add_keyword_token(TokenType::BangEqual);
                self.buf_reader_position = new_buffer_position;
                self.offset += 1;
            }
            _ => self.add_keyword_token(TokenType::Bang),
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
            _ => self.add_keyword_token(TokenType::Slash),
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
                    self.add_error(MushError::IncompleteString(IncompleteString::new(
                        string,
                        self.current_line,
                        self.offset,
                    )));
                    return Ok(());
                }
                '"' => {
                    self.add_lexeme_token(TokenType::String, &string);
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
                            _ => self.add_lexeme_token(TokenType::Float, &number),
                        }
                    }
                }
                _ => {
                    self.add_lexeme_token(TokenType::Interger, &number);
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

    fn add_keyword_token(&mut self, token_type: TokenType) {
        let token = KeywordToken::new(token_type, self.current_line, self.offset);
        self.tokens.push(Box::new(token));
    }

    fn add_lexeme_token(&mut self, token_type: TokenType, lexeme: &str) {
        let token = LexemeToken::new(token_type, lexeme, self.current_line, self.offset);
        self.tokens.push(Box::new(token));
    }

    pub fn has_errors(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn errors(&self) -> &[MushError] {
        &self.errors
    }
}
