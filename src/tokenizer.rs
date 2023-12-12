use std::{
    io::{Error, ErrorKind},
    process::exit,
};

#[derive(Debug, Clone)]
pub enum KeywordType {
    Let,
    Print,
}

#[derive(Debug, Clone)]
pub enum LiteralType {
    Int,
    String,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Assignment,
    Ident,
    Semicolon,
    Keyword(KeywordType),
    Literal(LiteralType),
}

#[derive(Debug, Clone)]
pub struct Token {
    type_type: TokenType,
    value: String,
}

impl Token {
    pub fn new(type_type: TokenType, value: String) -> Token {
        Token { type_type, value }
    }
}

pub struct Tokenizer {
    text: String,
    index: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(text: String) -> Tokenizer {
        Tokenizer {
            text,
            index: 0,
            tokens: Vec::new(),
        }
    }

    fn consume(&mut self) -> char {
        let cur = self.text.chars().nth(self.index).unwrap();
        self.index += 1;
        cur
    }

    fn peek(&self, offset: usize) -> Option<char> {
        if self.index + offset >= self.text.len() {
            None
        } else {
            Some(self.text.chars().nth(self.index + offset)?)
        }
    }

    pub fn tokenize(&mut self) -> std::io::Result<Vec<Token>> {
        let chars: Vec<char> = self.text.chars().collect();

        while self.peek(0) != None {
            let mut char = self.peek(0).unwrap();
            if char.is_whitespace() {
                self.consume();
                continue;
            } else if char == '#' {
                while char != '\n' {
                    self.consume();
                    char = self.peek(0).unwrap();
                }
                continue;
            } else if char == 'l' && Some('e') == self.peek(1) && Some('t') == self.peek(2) {
                self.tokens.push(Token::new(
                    TokenType::Keyword(KeywordType::Let),
                    "let".to_string(),
                ));
                self.consume();
                self.consume();
                self.consume();
            } else if char.is_ascii_alphabetic() {
                let mut builder = String::new();
                builder.push(char);
                self.consume();
                char = self.peek(0).unwrap();
                while char.is_ascii_alphanumeric() {
                    builder.push(char);
                    self.consume();
                    char = self.peek(0).unwrap();
                }
                self.tokens.push(Token::new(TokenType::Ident, builder));
            } else if char == '=' {
                self.tokens
                    .push(Token::new(TokenType::Assignment, "=".to_string()));
                self.consume();
            } else if char == ';' {
                self.tokens
                    .push(Token::new(TokenType::Semicolon, ";".to_string()));
                self.consume();
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Unexpected character: {:?}", char),
                ));
            }
        }

        self.index = 0;
        Ok(self.tokens.to_vec())
    }
}
