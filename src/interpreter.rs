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

#[derive(Debug, Clone, PartialEq)]
struct IdentValue {
    value: String,
    type_: LiteralType,
}

pub struct Interpreter {
    tokens: Vec<Token>,
    index: usize,
    mem: HashMap<String, IdentValue>,
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
                                    let ident_value_token = self.peek(3).unwrap();
                                    let TokenType::Literal(type_) = &ident_value_token.token_type
                                    else {
                                        return Err(Error::new(
                                            ErrorKind::Other,
                                            format!(
                                                "Expected literal type, got {:?}",
                                                ident_value_token
                                            ),
                                        ));
                                    };

                                    self.mem.insert(
                                        ident_token.value.clone(),
                                        IdentValue {
                                            value: ident_value_token.value.clone(),
                                            type_: type_.clone(),
                                        },
                                    );

                                    check_for_semicolon!(self, 3);

                                    self.consume_times(4)?;
                                }
                                TokenType::Colon => {
                                    let type_token = self.peek(3).unwrap();
                                    let next_token = self.peek(4).unwrap();
                                    match next_token.token_type {
                                        TokenType::Assignment => {
                                            let ident_value_token = self.peek(5).unwrap();
                                            let TokenType::Literal(type_) =
                                                &ident_value_token.token_type
                                            else {
                                                return Err(Error::new(
                                                    ErrorKind::Other,
                                                    format!(
                                                        "Expected literal type, got {:?}",
                                                        ident_value_token
                                                    ),
                                                ));
                                            };

                                            match type_token.value.as_str() {
                                                "string" => {
                                                    if type_.to_owned() != LiteralType::String {
                                                        return Err(Error::new(
                                                            ErrorKind::Other,
                                                            format!(
                                                                "Expected String, got {:?}: {:?}",
                                                                type_, ident_value_token.value
                                                            ),
                                                        ));
                                                    }

                                                    self.mem.insert(
                                                        ident_token.value.clone(),
                                                        IdentValue {
                                                            value: ident_value_token.value.clone(),
                                                            type_: type_.clone(),
                                                        },
                                                    );
                                                }
                                                "number" => {
                                                    if type_.to_owned() != LiteralType::Number {
                                                        return Err(Error::new(
                                                            ErrorKind::Other,
                                                            format!(
                                                                "Expected Number, got {:?}: {:?}",
                                                                type_, ident_value_token.value
                                                            ),
                                                        ));
                                                    }

                                                    self.mem.insert(
                                                        ident_token.value.clone(),
                                                        IdentValue {
                                                            value: ident_value_token.value.clone(),
                                                            type_: type_.clone(),
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    return Err(Error::new(
                                                        ErrorKind::Other,
                                                        format!("Unknown type {:?}", type_token),
                                                    ));
                                                }
                                            }

                                            check_for_semicolon!(self, 5);

                                            self.consume_times(2)?;
                                        }
                                        _ => match type_token.value.as_str() {
                                            "string" => {
                                                self.mem.insert(
                                                    ident_token.value.clone(),
                                                    IdentValue {
                                                        value: String::new(),
                                                        type_: LiteralType::String,
                                                    },
                                                );
                                            }
                                            "number" => {
                                                self.mem.insert(
                                                    ident_token.value.clone(),
                                                    IdentValue {
                                                        value: String::from("0"),
                                                        type_: LiteralType::Number,
                                                    },
                                                );
                                            }
                                            _ => {
                                                return Err(Error::new(
                                                    ErrorKind::Other,
                                                    format!("Unknown type {:?}", type_token),
                                                ));
                                            }
                                        },
                                    }

                                    self.consume_times(4)?;
                                }
                                _ => {}
                            };
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
                                        let Some(ident_value) = self.mem.get(&next_token.value)
                                        else {
                                            return Err(Error::new(
                                                ErrorKind::Other,
                                                format!(
                                                    "Unknown identifier {:?}",
                                                    next_token.value
                                                ),
                                            ));
                                        };
                                        match ident_value.type_ {
                                            LiteralType::String => {
                                                println!("{}", ident_value.value);
                                            }
                                            LiteralType::Number => {
                                                println!(
                                                    "{}",
                                                    ident_value
                                                        .value
                                                        .parse::<f64>()
                                                        .to_owned()
                                                        .unwrap()
                                                );
                                            }
                                        }
                                    }
                                    TokenType::Literal(LiteralType::String) => {
                                        println!("{}", next_token.value);
                                    }
                                    TokenType::Literal(LiteralType::Number) => {
                                        println!(
                                            "{}",
                                            next_token.value.parse::<f64>().to_owned().unwrap()
                                        );
                                    }
                                    _ => {
                                        continue;
                                    }
                                }
                                self.consume()?;
                            }
                            TokenType::Literal(LiteralType::String) => {
                                println!("{}", next_token.value);
                            }
                            TokenType::Literal(LiteralType::Number) => {
                                println!("{}", next_token.value);
                            }
                            TokenType::Assignment => {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    "'print' is not assignable",
                                ));
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
                    } else {
                        let next_token = self.peek(1).unwrap();
                        match next_token.token_type {
                            TokenType::Assignment => {
                                let ident_value_token = self.peek(2).unwrap();
                                let TokenType::Literal(type_) = &ident_value_token.token_type
                                else {
                                    return Err(Error::new(
                                        ErrorKind::Other,
                                        format!(
                                            "Expected literal type, got {:?}",
                                            ident_value_token
                                        ),
                                    ));
                                };

                                let Some(saved_value) = self.mem.get(&token.value) else {
                                    return Err(Error::new(
                                        ErrorKind::Other,
                                        format!("Unknown identifier {:?}", token.value),
                                    ));
                                };

                                if type_.to_owned() != saved_value.type_ {
                                    return Err(Error::new(
                                        ErrorKind::Other,
                                        format!(
                                            "Expected {:?}, got {:?}",
                                            type_, saved_value.type_
                                        ),
                                    ));
                                }

                                self.mem.insert(
                                    token.value.clone(),
                                    IdentValue {
                                        value: ident_value_token.value.clone(),
                                        type_: type_.clone(),
                                    },
                                );
                                check_for_semicolon!(self, 2);

                                self.consume_times(3)?;
                            }
                            TokenType::OpenParen => todo!(),
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("Unhandled token: {:?}", next_token),
                                ));
                            }
                        }
                    }
                }
                TokenType::Semicolon => {
                    self.consume()?;
                }
                _ => {
                    panic!("Unknown/Unhandled token type: {:?}", token.token_type);
                }
            }
        }

        Ok(())
    }
}
