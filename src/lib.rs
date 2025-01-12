#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use lexer::{Number, Token};

pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parse;

fn token_arr_to_number(numbers: &Vec<&Token>) -> f64 {
    let grouped_number = numbers
        .iter()
        .map(|t| {
            if let Token::Numeric(Number(fp)) = t {
                *fp
            } else {
                panic!("Error: somehow pushed a not-number in tokens_to_group.");
            }
        })
        .enumerate()
        .fold(0, |acc: i32, n: (usize, f64)| acc * 10 + n.1 as i32);

    grouped_number as f64
}
