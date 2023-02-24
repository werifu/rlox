use std::{collections::HashMap, vec};

use crate::token::{Token, TokenType};
pub struct Scanner {
    source_code: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source_code: String) -> Self {
        Self {
            source_code,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.scan_token() {
            match token.r#type {
                TokenType::Invalid => {
                    println!("[line {}]invalid token: {}", token.line, token.lexeme)
                }
                TokenType::Blank => {}
                _ => tokens.push(token),
            }
        }
        tokens.push(Token::new(TokenType::Eof, String::new(), self.line));
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        self.start = self.current;
        self.source_code
            .chars()
            .nth(self.current)
            .map(|ch| match ch {
                '(' => {
                    self.current += 1;
                    Token::new(TokenType::LeftParen, String::from("("), self.line)
                }
                '{' => {
                    self.current += 1;
                    Token::new(TokenType::LeftBrace, String::from("{"), self.line)
                }
                '}' => {
                    self.current += 1;
                    Token::new(TokenType::RightBrace, String::from("}"), self.line)
                }
                ')' => {
                    self.current += 1;
                    Token::new(TokenType::RightParen, String::from(")"), self.line)
                }
                ',' => {
                    self.current += 1;
                    Token::new(TokenType::Comma, String::from(","), self.line)
                }
                '.' => {
                    self.current += 1;
                    Token::new(TokenType::Dot, String::from("."), self.line)
                }
                '-' => {
                    self.current += 1;
                    Token::new(TokenType::Minus, String::from("-"), self.line)
                }
                '+' => {
                    self.current += 1;
                    Token::new(TokenType::Plus, String::from("+"), self.line)
                }
                ';' => {
                    self.current += 1;
                    Token::new(TokenType::Semicolon, String::from(";"), self.line)
                }
                '*' => {
                    self.current += 1;
                    Token::new(TokenType::Star, String::from("*"), self.line)
                }
                '/' => {
                    self.current += 1;
                    Token::new(TokenType::Slash, String::from("/"), self.line)
                }
                '!' => {
                    self.current += 1;
                    if self.source_code.chars().nth(self.current) == Some('=') {
                        self.current += 1;
                        Token::new(TokenType::BangEqual, String::from("!="), self.line)
                    } else {
                        Token::new(TokenType::Bang, String::from("!"), self.line)
                    }
                }
                '=' => {
                    self.current += 1;
                    if self.source_code.chars().nth(self.current) == Some('=') {
                        self.current += 1;
                        Token::new(TokenType::EqualEqual, String::from("=="), self.line)
                    } else {
                        Token::new(TokenType::Equal, String::from("="), self.line)
                    }
                }
                '>' => {
                    self.current += 1;
                    if self.source_code.chars().nth(self.current) == Some('=') {
                        self.current += 1;
                        Token::new(TokenType::GreaterEqual, String::from(">="), self.line)
                    } else {
                        Token::new(TokenType::Greater, String::from(">"), self.line)
                    }
                }
                '<' => {
                    self.current += 1;
                    if self.source_code.chars().nth(self.current) == Some('=') {
                        self.current += 1;
                        Token::new(TokenType::LessEqual, String::from("<="), self.line)
                    } else {
                        Token::new(TokenType::Less, String::from("<"), self.line)
                    }
                }
                '\n' => {
                    let token = Token::new(TokenType::Blank, String::from(ch), self.line);
                    self.current += 1;
                    self.line += 1;
                    token
                }
                ' ' | '\t' | '\r' => {
                    self.current += 1;
                    Token::new(TokenType::Blank, String::from(ch), self.line)
                }
                'A'..='Z' | 'a'..='z' => self.identifier(),
                '0'..='9' => self.number(),
                '"' => self.string(),
                invalid => {
                    self.current += 1;
                    Token::new(TokenType::Invalid, invalid.into(), self.line)
                }
            })
    }

    fn identifier(&mut self) -> Token {
        let mut token = String::new();
        while let Some(ch) = self.source_code.chars().nth(self.current) {
            if ch.is_alphanumeric() {
                token.push(ch);
                self.current += 1;
            } else {
                break;
            }
        }
        if let Some(preserved) = preserved_word(token.as_str(), self.line) {
            preserved
        } else {
            Token::new(TokenType::Identifier, token, self.line)
        }
    }

    fn number(&mut self) -> Token {
        let mut token = String::new();
        let mut dot_consumed = false;
        while let Some(ch) = self.source_code.chars().nth(self.current) {
            if ch.is_numeric() {
                token.push(ch);
                self.current += 1;
            } else if ch == '.' && !dot_consumed {
                dot_consumed = true;
                token.push(ch);
                self.current += 1;
            } else {
                break;
            }
        }
        // error number parse handle
        Token::new(TokenType::Number, token, self.line)
    }

    /// expect to parse a string literal like "aaa"
    /// do not support \
    fn string(&mut self) -> Token {
        let mut token = String::new();
        // skip the first quote
        self.current += 1;
        while let Some(ch) = self.source_code.chars().nth(self.current) {
            if ch == '\n' {
                self.line += 1;
            } else if ch != '"' {
                token.push(ch);
            }
            self.current += 1;

            // out of this loop when meeting the second quote
            if ch == '"' {
                break;
            }
        }
        Token::new(TokenType::String, token, self.line)
    }
}

fn preserved_word(token: &str, line: usize) -> Option<Token> {
    match token {
        "and" => Some(Token::new(TokenType::And, "and".to_string(), line)),
        "class" => Some(Token::new(TokenType::Class, "class".to_string(), line)),
        "else" => Some(Token::new(TokenType::Else, "else".to_string(), line)),
        "false" => Some(Token::new(TokenType::False, "false".to_string(), line)),
        "for" => Some(Token::new(TokenType::For, "for".to_string(), line)),
        "func" => Some(Token::new(TokenType::Func, "func".to_string(), line)),
        "if" => Some(Token::new(TokenType::If, "if".to_string(), line)),
        "nil" => Some(Token::new(TokenType::Nil, "nil".to_string(), line)),
        "or" => Some(Token::new(TokenType::Or, "or".to_string(), line)),
        "print" => Some(Token::new(TokenType::Print, "print".to_string(), line)),
        "return" => Some(Token::new(TokenType::Return, "return".to_string(), line)),
        "super" => Some(Token::new(TokenType::Super, "super".to_string(), line)),
        "this" => Some(Token::new(TokenType::This, "this".to_string(), line)),
        "true" => Some(Token::new(TokenType::True, "true".to_string(), line)),
        "var" => Some(Token::new(TokenType::Var, "var".to_string(), line)),
        "while" => Some(Token::new(TokenType::While, "while".to_string(), line)),
        _ => None,
    }
}

#[test]
fn test_run() {
    let source_code = "var id = 114.514;";

    let tokens = Scanner::new(source_code.to_string()).scan_tokens();
    let should_be = vec![
        Token::new(TokenType::Var, "var".to_string(), 1),
        Token::new(TokenType::Identifier, "id".to_string(), 1),
        Token::new(TokenType::Equal, "=".to_string(), 1),
        Token::new(TokenType::Number, "114.514".to_string(), 1),
        Token::new(TokenType::Semicolon, ";".to_string(), 1),
        Token::new(TokenType::Eof, String::new(), 1),
    ];
    assert_eq!(tokens, should_be);

    let source_code = "while (a == 114@) {\n var b = \"while\";\n }\n";
    let tokens = Scanner::new(source_code.to_string()).scan_tokens();
    let should_be = vec![
        Token::new(TokenType::While, "while".to_string(), 1),
        Token::new(TokenType::LeftParen, "(".to_string(), 1),
        Token::new(TokenType::Identifier, "a".to_string(), 1),
        Token::new(TokenType::EqualEqual, "==".to_string(), 1),
        Token::new(TokenType::Number, "114".to_string(), 1),
        Token::new(TokenType::RightParen, ")".to_string(), 1),
        Token::new(TokenType::LeftBrace, "{".to_string(), 1),
        Token::new(TokenType::Var, "var".to_string(), 2),
        Token::new(TokenType::Identifier, "b".to_string(), 2),
        Token::new(TokenType::Equal, "=".to_string(), 2),
        Token::new(TokenType::String, "while".to_string(), 2),
        Token::new(TokenType::Semicolon, ";".to_string(), 2),
        Token::new(TokenType::RightBrace, "}".to_string(), 3),
        Token::new(TokenType::Eof, String::new(), 4),
    ];
    assert_eq!(tokens, should_be);
}
