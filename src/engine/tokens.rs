trait StringToken {
    fn as_str(&self) -> &str;
}
trait CharToken {
    fn as_char(&self) -> char;
}
trait MatchToken {}

#[derive(Debug)]
pub enum Token {
    EndOfFile,
    SingleToken(SingleToken),
    CompositeToken(CompositeToken),
    KeywordToken(KeywordToken),
    LexemeToken(LexemeToken),
}

impl Token {
    pub fn match_char(next: char) -> Option<Self> {
        match next {
            '(' => Some(Token::SingleToken(SingleToken::LeftParen)),
            ')' => Some(Token::SingleToken(SingleToken::RightParen)),
            '{' => Some(Token::SingleToken(SingleToken::LeftCurl)),
            '}' => Some(Token::SingleToken(SingleToken::RightCurl)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum SingleToken {
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
    Bang,
    Equal,
    Greater,
    Less,
}
impl CharToken for SingleToken {
    fn as_char(&self) -> char {
        match self {
            SingleToken::LeftParen => '(',
            SingleToken::RightParen => ')',
            SingleToken::LeftCurl => '{',
            SingleToken::RightCurl => '}',
            SingleToken::Comma => ',',
            SingleToken::Dot => '.',
            SingleToken::Minus => '-',
            SingleToken::Plus => '+',
            SingleToken::SemiColon => ';',
            SingleToken::NewLine => '\n',
            SingleToken::Slash => '/',
            SingleToken::Star => '*',
            SingleToken::Bang => '!',
            SingleToken::Equal => '=',
            SingleToken::Greater => '>',
            SingleToken::Less => '<',
        }
    }
}

#[derive(Debug)]
pub enum CompositeToken {
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,
}
impl StringToken for CompositeToken {
    fn as_str(&self) -> &str {
        match self {
            CompositeToken::BangEqual => "!=",
            CompositeToken::EqualEqual => "==",
            CompositeToken::GreaterEqual => ">=",
            CompositeToken::LessEqual => "<=",
        }
    }
}

#[derive(Debug)]
pub enum KeywordToken {
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
}
impl StringToken for KeywordToken {
    fn as_str(&self) -> &str {
        match self {
            KeywordToken::And => "and",
            KeywordToken::Fn => "fn",
            KeywordToken::For => "for",
            KeywordToken::If => "if",
            KeywordToken::None => "None",
            KeywordToken::Or => "Or",
            KeywordToken::Return => "return",
            KeywordToken::True => "True",
            KeywordToken::False => "False",
            KeywordToken::Let => "let",
            KeywordToken::While => "while",
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
impl StringToken for LexemeToken {
    fn as_str(&self) -> &str {
        match self {
            LexemeToken::Identifier(s) => s,
            LexemeToken::String(s) => s,
            LexemeToken::Interger(s) => s,
            LexemeToken::Float(s) => s,
        }
    }
}
