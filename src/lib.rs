#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use error::ParseError;
use nom_locate::LocatedSpan;

pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parse;
pub mod util;

type Span<'a> = LocatedSpan<&'a str>;
pub type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

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
