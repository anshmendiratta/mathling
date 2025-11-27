use std::collections::{hash_map::Iter, HashMap};

use crate::{
    lexer::{BinOp, Expr},
    math_lexing::MathLexer,
    Token, TokenType,
};

pub struct SymbolTable<O: Clone> {
    pub variables: HashMap<String, O>,
}

impl<O: Clone> SymbolTable<O> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn lookup(&self, id: &str) -> Option<&O> {
        self.variables.get(id)
    }

    pub fn add(&mut self, id: &str, val: O) {
        self.variables.insert(id.to_owned(), val);
    }
}

impl SymbolTable<Vec<Token>> {
    pub fn simplify(&self) -> SymbolTable<f32> {
        let mut fp_symbol_table = SymbolTable::<f32>::new();

        for (id, tokens) in self.variables.clone() {
            // let (_, tokens) = MathLexer::new(e).lex().unwrap();
            let mut stack: Vec<Token> = vec![];
            for token in tokens {
                match token {
                    Token {
                        token_type: TokenType::Fp(ref n),
                        ..
                    } => stack.push(token),
                    Token {
                        token_type: TokenType::BinOp(op),
                        ..
                    } => {
                        // Made `mut` so they can be made into floats if the operator is division.
                        let mut y = match stack.pop() {
                            Some(Token {
                                token_type: TokenType::Fp(n),
                                ..
                            }) => n,
                            _ => {
                                panic!("Ill-formed expression.")
                            }
                        };
                        let mut x = match stack.pop() {
                            Some(Token {
                                token_type: TokenType::Fp(n),
                                ..
                            }) => n,
                            _ => panic!("Ill-formed expression or variable reference."),
                        };
                        match op {
                            BinOp::Plus => {
                                stack.push(Token {
                                    token_type: TokenType::Fp(x + y),
                                    location_col: None,
                                });
                            }
                            BinOp::Minus => {
                                stack.push(Token {
                                    token_type: TokenType::Fp(x - y),
                                    location_col: None,
                                });
                            }
                            BinOp::Times => {
                                stack.push(Token {
                                    token_type: TokenType::Fp(x * y),
                                    location_col: None,
                                });
                            }
                            BinOp::Divide => {
                                stack.push(Token {
                                    token_type: TokenType::Fp(x / y),
                                    location_col: None,
                                });
                            }
                            BinOp::Equal => unreachable!(),
                        }
                    }
                    _ => (),
                }
            }

            assert!(
            stack.len() == 1,
            "Evaluator stack is not of length 1. Either all variables introduced were not used, or there exists an ill-formed expression."
        );
            match stack.first().unwrap() {
                Token {
                    token_type: TokenType::Fp(n),
                    ..
                } => fp_symbol_table.variables.insert(id, *n),
                _ => panic!("Error: After eval, last token is NOT a number."),
            };
        }

        fp_symbol_table
    }
}
