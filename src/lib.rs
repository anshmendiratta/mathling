#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use crate::lexer::Token;
use lexer::{Number, TokenKind};

pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parse;

fn token_arr_to_number(numbers: &[Token]) -> (f64, usize) {
    assert!(!numbers.is_empty());
    let first_col = numbers[0].col;
    let grouped_number = numbers
        .iter()
        .map(|t| {
            if let Token {
                kind: TokenKind::Numeric(Number(fp)),
                ..
            } = t
            {
                *fp
            } else {
                panic!("Error: somehow pushed a not-number in tokens_to_group.");
            }
        })
        .enumerate()
        .fold(0, |acc: i32, n: (usize, f64)| acc * 10 + n.1 as i32);

    (grouped_number as f64, first_col)
}
