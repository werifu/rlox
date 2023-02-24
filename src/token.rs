use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            line,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    Invalid,
    Blank,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Semicolon => ";",
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::Bang => "!",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Identifier => "Identifier",
            TokenType::String => "String",
            TokenType::Number => "Number",
            TokenType::And => "&&",
            TokenType::Class => "class",
            TokenType::Else => "else",
            TokenType::False => "false",
            TokenType::Func => "func",
            TokenType::For => "for",
            TokenType::If => "if",
            TokenType::Nil => "nil",
            TokenType::Or => "or",
            TokenType::Print => "print",
            TokenType::Return => "return",
            TokenType::Super => "super",
            TokenType::This => "this",
            TokenType::True => "true",
            TokenType::Var => "var",
            TokenType::While => "while",
            TokenType::Eof => "Eof",
            TokenType::Invalid => "Invalid",
            TokenType::Blank => "Blank",
        };
        write!(f, "{}", str.to_string())
    }
}
