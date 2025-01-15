#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use crate::lexer::Token;
use lexer::{Number, TokenKind};

pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parse;

fn token_arr_to_number(numbers: &Vec<Token>) -> (f64, usize) {
    assert!(numbers.len() != 0);
    let first_col = numbers[0].col;
    let grouped_number = numbers
        .iter()
        .map(|t| {
            if let Token { kind: TokenKind::Numeric(Number(fp)), ..} = t {
                *fp
            } else {
                panic!("Error: somehow pushed a not-number in tokens_to_group.");
            }
        })
        .enumerate()
        .fold(0, |acc: i32, n: (usize, f64)|
            /* can do 10x each iter because n.1 is always a single-digit number */ 
            acc * 10 + n.1 as i32
        );

    (grouped_number as f64, first_col)
}

pub fn token_arr_to_fp(tokens: &[Token; 5]) -> Token {
    match tokens {
                    [a, b, Token {
                        kind: TokenKind::Period,
                        ..
                    }, d, _] => {
                        tokens_bind.push(a.clone());
                    }
                    [a, Token {
                        kind: TokenKind::Period,
                        col,
                    }, c] => {
                        let (a, c) = match (a, c) {
                            (
                                Token {
                                    kind: TokenKind::Numeric(Number(n_1)),
                                    ..
                                },
                                Token {
                                    kind: TokenKind::Numeric(Number(n_2)),
                                    ..
                                },
                            ) => (n_1, n_2),
                            _ => panic!(" "),
                        };
                        let fp = a + c / (10_f64.powf(c.to_string().len() as f64));
                        let fp_tk = Token {
                            kind: TokenKind::Numeric(Number(fp)),
                            col: *col,
                        };
                        tokens_bind.push(fp_tk);
                    }
                    [Token {
                        kind: TokenKind::Period,
                        ..
                    }, b, c] => {
                        tokens_bind.push(c.clone());
                    }
                    [a, b, c] => {
                        tokens_bind.push(a.clone());
                        tokens_bind.push(b.clone());
                        tokens_bind.push(c.clone());
                    }
                    _ => (),    }
}
