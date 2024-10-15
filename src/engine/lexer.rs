use std::{
    fmt::Display,
    io::{BufRead, BufReader, Error, Read, Seek, SeekFrom},
};

use super::diagnostics::error::{MushError, UnknownCharacter};

const BUFFER_SIZE: usize = 1;

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

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
    offset: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: i32, offset: i32) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_owned(),
            line,
            offset,
        }
    }

    fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    fn lexeme(&self) -> &str {
        self.lexeme.as_ref()
    }

    fn line(&self) -> i32 {
        self.line
    }

    fn offset(&self) -> i32 {
        self.offset
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme())
    }
}

pub struct Scanner<R>
where
    R: Seek + Read,
{
    buf_reader: BufReader<R>,
    tokens: Vec<Token>,
    errors: Vec<MushError>,

    current_line: i32,
    offset: i32,
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
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), Error> {
        self.buf_reader.rewind()?;

        let mut is_done = self.is_done()?;
        while !is_done {
            is_done = self.is_done()?;
            self.scan_token()?;
        }

        self.add_token(TokenType::EndOfFile, "");

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance()?;

        match c.as_str() {
            "(" => self.add_token(TokenType::LeftParen, "("),
            ")" => self.add_token(TokenType::RightParen, ")"),
            "\n" => {
                self.add_token(TokenType::NewLine, "\n");
                self.current_line += 1;
                self.offset = 0;
            }
            _ => {}
        };
        Ok(())
    }

    fn advance(&mut self) -> Result<String, Error> {
        self.buf_reader.seek(SeekFrom::Current(1))?;
        let mut buffer = [0; BUFFER_SIZE];
        let n = self.buf_reader.read(&mut buffer)?;
        let character = &buffer[..n];
        self.offset += 1;
        match std::str::from_utf8(character) {
            Ok(c) => Ok(c.to_string()),
            Err(_err) => Ok("".to_string()),
        }
    }

    fn is_done(&mut self) -> Result<bool, Error> {
        let buffer = self.buf_reader.fill_buf()?;
        Ok(buffer.is_empty())
    }

    fn add_error(&mut self, mush_error: MushError) {
        self.errors.push(mush_error)
    }
    fn add_token(&mut self, token_type: TokenType, lexeme: &str) {
        let token = Token::new(token_type, lexeme, self.current_line, self.offset);
        self.tokens.push(token)
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}
