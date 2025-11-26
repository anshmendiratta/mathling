#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use error::ParseError;
use nom::{Input, Parser};
use nom_locate::LocatedSpan;

use crate::lexer::BinOp;

pub mod codegen;
pub mod error;
pub mod lexer;
pub mod math_lexing;
pub mod parse;
pub mod symbols;
pub mod util;

type Span<'a> = LocatedSpan<&'a str>;
pub type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    location_col: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Fp(f32),
    Id(String),
    BinOp(BinOp),
    LeftParen,
    RightParen,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fp(l0), Self::Fp(r0)) => l0 == r0,
            (Self::Id(l0), Self::Id(r0)) => l0 == r0,
            (Self::BinOp(l0), Self::BinOp(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

// fn token_arr_to_number(numbers: &[Token]) -> (f32, usize) {
//     assert!(!numbers.is_empty());
//     let first_col = numbers[0].column;
//     let grouped_number = numbers
//         .iter()
//         .map(|t| {
//             if let Token {
//                 kind: TokenKind::Numeric(Number(fp)),
//                 ..
//             } = t
//             {
//                 *fp
//             } else {
//                 panic!("Error: somehow pushed a not-number in tokens_to_group.");
//             }
//         })
//         .enumerate()
//         .fold(0, |acc: i32, n: (usize, f32)| acc * 10 + n.1 as i32);

//     (grouped_number as f32, first_col)
// }

// fn alphabetical_arr_to_identifier(letters: &[Token]) -> Token {
//     let col_to_use = letters.first().unwrap().col();
//     let identifier = letters
//         .iter()
//         .map(|t| {
//             if let TokenKind::Alphabetical(c) = t.kind {
//                 c.to_string()
//             } else {
//                 unreachable!()
//             }
//         })
//         .collect::<Vec<String>>()
//         .join("");

//     Token {
//         kind: TokenKind::Identifier(identifier),
//         column: col_to_use,
//     }
// }
