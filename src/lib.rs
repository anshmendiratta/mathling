#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use crate::lexer::Token;
use lexer::{Number, TokenKind};

pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parse;

fn token_arr_to_number(numbers: &Vec<&Token>) -> f64 {
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
        .fold(0, |acc: i32, n: (usize, f64)|/* can do 10x each iter because n.1 is always a single-digit number */ acc * 10 + n.1 as i32);

    grouped_number as f64
}
