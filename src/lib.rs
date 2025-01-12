#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![feature(slice_as_array)]

use lexer::{Number, Token};

pub mod codegen;
pub mod parse;

pub mod lexer {
    use crate::token_arr_to_number;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        LeftParen,
        RightParen,
        Numeric(Number),
        Whitespace,
        Op(Operator),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Number {
        // TOOD: Add support for later.
        FloatingPoint(f64),
    }

    impl std::fmt::Display for Number {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Number::FloatingPoint(fp) => f.write_str(&fp.to_string()),
                // Number::Integer(n) => f.write_str(&n.to_string()),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Operator {
        Plus,
        Minus,
        Asterisk,
        // /
        Slash,
    }

    impl Operator {
        pub fn has_greater_precedence_than(&self, other_op: Self) -> bool {
            match self {
                Operator::Plus | Operator::Minus => false,
                Operator::Asterisk => match other_op {
                    Operator::Slash | Operator::Asterisk => false,
                    _ => true,
                },
                Operator::Slash => {
                    if let Operator::Slash = other_op {
                        false
                    } else {
                        true
                    }
                }
            }
        }
    }

    pub struct Lexer<'a> {
        src: &'a str,
        current_token: Option<char>,
        rest: &'a str,
    }

    impl<'a> Lexer<'a> {
        pub fn new(expr_as_string: &'a str) -> Self {
            Self {
                src: expr_as_string,
                current_token: None,
                rest: expr_as_string,
            }
        }

        pub fn lex(mut self) -> Vec<Token> {
            let mut tokens = vec![];

            self.advance();
            while self.current_token.is_some() {
                match self.tokenize_character(self.current_token.unwrap()) {
                    Some(t) => {
                        // Ignore whitespace as a token.
                        if let Token::Whitespace = t {
                            // Need this so does not end in an infinite loop.
                            self.advance();
                            continue;
                        }
                        tokens.push(t)
                    }
                    _ => {
                        let column = self.src.find(self.current_token.unwrap()).unwrap();
                        panic!(
                            "Unexpected token: {:?} at col {} ",
                            self.current_token.unwrap(),
                            column + 1 /* +1 for 1-indexing */
                        );
                    }
                };

                self.advance();
            }

            // Group adjacent `Numbers` into a single one.
            // Similar to the evaluation of RPN, this adds numbers to a vector until it reaches an operator or another token.
            // Then, it tries to unify each digit into one number and pushes it to a final token vector along with the operator.
            let mut grouped_tokens: Vec<Token> = vec![];
            let mut tokens_to_group: Vec<&Token> = vec![];
            let mut tokens_to_eat = tokens.len() - 1;

            for token in &tokens {
                if tokens_to_eat == 0 {
                    tokens_to_group.push(token);
                    let grouped_number = token_arr_to_number(&tokens_to_group);
                    grouped_tokens.push(Token::Numeric(Number::FloatingPoint(grouped_number)));
                    break;
                };
                match token {
                    Token::Numeric(_) => tokens_to_group.push(token),
                    _ => {
                        let grouped_number = token_arr_to_number(&tokens_to_group);
                        grouped_tokens.push(Token::Numeric(Number::FloatingPoint(grouped_number)));
                        grouped_tokens.push(token.clone());
                        tokens_to_group.clear();
                    }
                }
                tokens_to_eat -= 1;
            }

            grouped_tokens
        }

        fn tokenize_character(&mut self, character: char) -> Option<Token> {
            match character {
                '(' => return Some(Token::LeftParen),
                ')' => return Some(Token::RightParen),
                '0'..='9' => {
                    return Some(Token::Numeric(Number::FloatingPoint(
                        character.to_digit(10).unwrap() as f64,
                    )))
                }
                '+' => return Some(Token::Op(Operator::Plus)),
                '-' => return Some(Token::Op(Operator::Minus)),
                '*' => return Some(Token::Op(Operator::Asterisk)),
                '/' => return Some(Token::Op(Operator::Slash)),
                ' ' => return Some(Token::Whitespace),
                _ => None,
            }
        }

        fn advance(&mut self) -> Option<char> {
            if self.rest.as_bytes().len() >= 1 {
                self.current_token = Some(self.rest.bytes().nth(0).unwrap() as char);
                self.rest = &self.rest[1..];
                return self.current_token;
            } else {
                self.current_token = None;
                return None;
            }
        }

        // fn tokenize(self) -> ASTNode {}
    }
}

fn token_arr_to_number(numbers: &Vec<&Token>) -> f64 {
    let grouped_number = numbers
        .iter()
        .map(|t| {
            if let Token::Numeric(Number::FloatingPoint(fp)) = t {
                fp.clone()
            } else {
                panic!("Error: somehow pushed a not-number in tokens_to_group.");
            }
        })
        .enumerate()
        .fold(0, |acc: i32, n: (usize, f64)| {
            let length = numbers.len() - 1;
            acc * 10 + n.1 as i32
            // acc + n.1 as i32 * 10_i32.pow(length as u32 - n.0 as u32)
        });

    grouped_number as f64
}
