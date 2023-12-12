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

    fn get_ident_value_token(&self, token: &Token) -> Option<&Token> {
        self.mem.get(&token.value)
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

                                    self.consume_times(4)?;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                TokenType::Ident => {
                    if token.value == "print" {
                        let next_token = self.peek(1).unwrap();
                        match next_token.token_type {
                            TokenType::OpenParen => {
                                let next_token = self.peek(2).unwrap();
                                match next_token.token_type {
                                    TokenType::Ident => {
                                        let Some(ident_value_token) =
                                            self.get_ident_value_token(next_token)
                                        else {
                                            return Err(Error::new(
                                                ErrorKind::Other,
                                                format!("Unknown identifier: {:?}", next_token),
                                            ));
                                        };
                                        println!("{}", ident_value_token.value);
                                    }
                                    TokenType::Literal(LiteralType::String) => {
                                        println!("{}", next_token.value);
                                    }
                                    TokenType::Literal(LiteralType::Number) => {
                                        println!("{}", next_token.value);
                                    }
                                    _ => {
                                        continue;
                                    }
                                }
                                self.consume()?;
                                // let next_token = self.peek(3).unwrap();
                                // match next_token.token_type {
                                //     TokenType::CloseParen => {
                                //         check_for_semicolon!(self, 3);
                                //     }
                                //     _ => {
                                //         return Err(Error::new(
                                //             ErrorKind::Other,
                                //             format!("Expected close paren, got {:?}", next_token),
                                //         ));
                                //     }
                                // };
                            }
                            TokenType::Literal(LiteralType::String) => {
                                println!("{}", next_token.value);
                            }
                            TokenType::Literal(LiteralType::Number) => {
                                println!("{}", next_token.value);
                            }
                            _ => {}
                        }
                        let next_token = self.peek(2).unwrap();
                        match next_token.token_type {
                            TokenType::CloseParen => {
                                check_for_semicolon!(self, 2);
                            }
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("Expected close paren, got {:?}", next_token),
                                ));
                            }
                        };
                        self.consume_times(3)?;
                    } else if false {
                        todo!()
                    }
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
