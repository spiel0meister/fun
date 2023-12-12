use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use super::tokenizer::*;

macro_rules! check_for_semicolon {
    ($interpreter:ident, $peek_depth:literal) => {
        let next_token = $interpreter.peek($peek_depth + 1).unwrap();
        match next_token.token_type {
            TokenType::Semicolon => {}
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Expected semicolon, got {:?}", next_token),
                ));
            }
        }
    };
}

pub struct Interpreter {
    tokens: Vec<Token>,
    index: usize,
    mem: HashMap<String, Token>,
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            mem: HashMap::new(),
        }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        if self.index + offset >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.index + offset])
        }
    }

    fn consume(&mut self) -> std::io::Result<&Token> {
        let token = &self.tokens[self.index];
        self.index += 1;
        Ok(token)
    }

    fn consume_times(&mut self, times: usize) -> std::io::Result<()> {
        for _ in 0..times {
            self.consume()?;
        }

        Ok(())
    }

    pub fn interpret(&mut self) -> std::io::Result<()> {
        while self.peek(0).is_some() {
            let token = self.peek(0).unwrap();
            match token.token_type {
                TokenType::Keyword(KeywordType::Let) => {
                    let ident_token = self.peek(1).unwrap();
                    match ident_token.token_type {
                        TokenType::Ident => {
                            let next_token = self.peek(2).unwrap();
                            match next_token.token_type {
                                TokenType::Assignment => {
                                    let ident_value = self.peek(3).unwrap();
                                    self.mem
                                        .insert(ident_token.value.clone(), ident_value.clone());
                                    check_for_semicolon!(self, 3);

                                    self.consume_times(5)?;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                TokenType::Keyword(KeywordType::Print) => {
                    let ident = self.peek(1).unwrap();
                    let Some(ident_value) = self.mem.get(&ident.value) else {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Unknown identifier: {:?}", ident.value),
                        ));
                    };
                    match ident_value.token_type {
                        TokenType::Literal(LiteralType::Number) => {
                            println!("{}", ident_value.value.parse::<f32>().unwrap());
                        }
                        TokenType::Literal(LiteralType::String) => {
                            println!("{}", ident_value.value);
                        }
                        _ => {
                            panic!()
                        }
                    };
                    check_for_semicolon!(self, 1);
                    self.consume_times(3)?;
                }
                TokenType::Semicolon => {
                    self.consume()?;
                }
                _ => {
                    panic!("Unhandled token type: {:?}", token.token_type);
                }
            }
        }

        Ok(())
    }
}
