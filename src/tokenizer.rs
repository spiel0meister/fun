use std::io::{Error, ErrorKind};

macro_rules! keyword_case {
    ($tokenizer:ident, $keyword:literal, $keyword_type:expr) => {
        add_token!(
            $tokenizer,
            TokenType::Keyword($keyword_type),
            $keyword.to_string()
        );
        $tokenizer.consume_times($keyword.len());
    };
}

macro_rules! add_token {
    ($tokenizer:ident, $token_type:expr, $token_value:expr) => {
        $tokenizer
            .tokens
            .push(Token::new($token_type, $token_value));
    };
}

#[derive(Debug, Clone)]
pub enum KeywordType {
    Let,
    None,
}

#[derive(Debug, Clone)]
pub enum LiteralType {
    Number,
    String,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Assignment,
    Ident,
    Semicolon,
    OpenParen,
    CloseParen,
    Keyword(KeywordType),
    Literal(LiteralType),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Self { token_type, value }
    }
}

pub struct Tokenizer {
    text: String,
    index: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Self {
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

    fn consume_times(&mut self, times: usize) -> () {
        for _ in 0..times {
            self.consume();
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        if self.index + offset >= self.text.len() {
            None
        } else {
            Some(self.text.chars().nth(self.index + offset)?)
        }
    }

    fn spells_out(&mut self, keyword: &str) -> bool {
        for (i, c) in keyword.chars().enumerate() {
            if Some(c) != self.peek(i) {
                return false;
            }
        }
        true
    }

    fn create_literal(&mut self, literal_type: LiteralType) -> std::io::Result<Token> {
        let Some(mut char) = self.peek(0) else {
            return Err(Error::new(ErrorKind::Other, "No char"));
        };
        let mut builder = String::new();
        match literal_type {
            LiteralType::String => {
                builder.push(char);
                self.consume();
                char = self.peek(0).unwrap();
                while char != '"' {
                    builder.push(char);
                    self.consume();
                    char = self.peek(0).unwrap();
                }
                self.consume();
                Ok(Token::new(TokenType::Literal(LiteralType::String), builder))
            }
            LiteralType::Number => {
                if char == '.' {
                    builder.push('0');
                } else {
                    builder.push(char);
                }
                self.consume();
                char = self.peek(0).unwrap();
                while char.is_ascii_digit() || char == '.' {
                    if char == '.' && builder.contains('.') {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Multiple decimal points in number",
                        ));
                    }
                    builder.push(char);
                    self.consume();
                    char = self.peek(0).unwrap();
                }
                Ok(Token::new(TokenType::Literal(LiteralType::Number), builder))
            }
        }
    }

    pub fn tokenize(&mut self) -> std::io::Result<Vec<Token>> {
        self.tokens = Vec::new();
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
            } else if self.spells_out("let") {
                keyword_case!(self, "let", KeywordType::Let);
            } else if self.spells_out("none") {
                keyword_case!(self, "none", KeywordType::None);
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
                add_token!(self, TokenType::Ident, builder);
            } else if char == '=' {
                add_token!(self, TokenType::Assignment, "=".to_string());
                self.consume();
            } else if char == '"' {
                self.consume();
                let res = self.create_literal(LiteralType::String)?;
                self.tokens.push(res);
            } else if char.is_ascii_digit() || char == '.' {
                let res = self.create_literal(LiteralType::Number)?;
                self.tokens.push(res);
            } else if char == ';' {
                add_token!(self, TokenType::Semicolon, ";".to_string());
                self.consume();
            } else if char == '(' {
                add_token!(self, TokenType::OpenParen, "(".to_string());
                self.consume();
            } else if char == ')' {
                add_token!(self, TokenType::CloseParen, ")".to_string());
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
