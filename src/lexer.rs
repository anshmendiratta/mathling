#[allow(unused_imports)]
use std::{collections::HashMap, fmt::Write, ops::Range};

use nom::{
    branch::{alt, Choice},
    bytes::{is_a, is_not, tag, take_until, take_while},
    character::{
        complete::{alpha1, alphanumeric0},
        digit1, one_of,
    },
    combinator::{map_res, recognize, verify},
    error::Error,
    multi::{many0, separated_list0},
    sequence::separated_pair,
    AsChar, FindToken, Input, Parser,
};

use crate::{error::ParseError, IResult, Span};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Token {
//     pub kind: TokenKind,
//     pub column: usize,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum TokenKind {
//     // For values.
//     LeftParen,
//     RightParen,
//     Numeric(Number),
//     Whitespace,
//     Period,
//     Op(BinOp),
//     // For parts of variables. Do not use directly in parser.
//     Alphabetical(char),
//     // For identifier.
//     Identifier(String),
//     Equal,
//     Semicolon, // To separate statements.
// }

// impl std::fmt::Display for TokenKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TokenKind::LeftParen => f.write_str("("),
//             TokenKind::RightParen => f.write_str(")"),
//             TokenKind::Numeric(number) => f.write_str(&number.to_string()),
//             TokenKind::Whitespace => f.write_str(" "),
//             TokenKind::Op(operator) => f.write_str(&operator.to_string()),
//             TokenKind::Period => f.write_str("."),
//             TokenKind::Alphabetical(c) => f.write_char(*c),
//             TokenKind::Equal => f.write_str("="),
//             TokenKind::Semicolon => f.write_str(";"),
//             TokenKind::Identifier(id) => f.write_str(id),
//         }
//     }
// }

// impl Token {
//     pub fn kind(&self) -> TokenKind {
//         let Token { kind: k, .. } = self;
//         k.clone()
//     }

//     pub fn col(&self) -> usize {
//         let Token { column: c, .. } = self;
//         c.clone()
//     }
// }

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Value(fp) => f.write_str(&fp.to_string()),
            Expr::Id(i) => f.write_str(i),
            Expr::BinOp(expr) => f.write_str(expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
}

impl BinOp {
    pub fn has_greater_precedence_than(&self, other_op: &Self) -> bool {
        match self {
            BinOp::Plus | BinOp::Minus => false,
            BinOp::Times => !matches!(other_op, BinOp::Divide | BinOp::Times),
            BinOp::Divide => !matches!(other_op, BinOp::Divide),
            BinOp::Equal => false,
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Plus => f.write_str("+"),
            BinOp::Minus => f.write_str("-"),
            BinOp::Times => f.write_str("*"),
            BinOp::Divide => f.write_str("/"),
            BinOp::Equal => f.write_str("="),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assign(String, Expr),
    Print(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Value(f32),
    Id(String),
    BinOp(String),
}

pub struct Lexer<'a> {
    src: Span<'a>,
    // rest: Span<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(expr_as_string: &'a str) -> Self {
        Self {
            src: Span::new(expr_as_string),
            // rest: Span::new(expr_as_string),
        }
    }

    pub fn lex(&'_ mut self) -> IResult<'_, Vec<Statement>> {
        let (_, statements) = self.parse_all()?;
        dbg!(&statements);
        Ok((Span::new(""), vec![]))
    }

    fn parse_all(&'_ mut self) -> IResult<'_, Vec<Statement>> {
        let mut statements: Vec<Statement> = [].into();
        let mut input = self.src;
        let mut capture;
        while !input.is_empty() {
            (input, capture) = take_until(";").parse(input)?;
            let (_, statement) = self.parse_statement(capture)?;
            statements.push(statement);
            (input, _) = tag(";").parse(input)?;
        }

        // let (input, statements) = map_res(separated_list0(tag(";"), is_not(";")), |statements| {
        //     Ok::<Vec<Statement>, Error<Span<'a>>>(
        //         statements
        //             .iter()
        //             .map(|s| self.parse_statement(*s))
        //             .collect(),
        //     )
        // })
        // .parse(self.src)?;

        Ok((Span::new(""), statements))
    }

    fn parse_statement(&'_ self, input: Span<'a>) -> IResult<'_, Statement> {
        let allowed_syntax = many0(alt((
            alphanumeric0,
            is_a::<&str, Span<'a>, ParseError>(" +-*/()"),
        )));
        if let Ok((input, (id, val))) =
            separated_pair(allowed_syntax, tag("="), allowed_syntax).parse(input)
        {
            let val = val
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("");
            let id = id
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("");
            let value_expr = self.parse_expr(Span::new(&val));
            Ok((input, Statement::Assign(id.to_string(), value_expr)))
        } else {
            let mut allowed_syntax =
                recognize(alt((alphanumeric0::<Span<'a>, ParseError>, is_a(" +-*/;"))));
            let (_, input) = allowed_syntax.parse(input)?;
            let print_expr = self.parse_expr(input);
            Ok((input, Statement::Print(print_expr)))
        }
    }

    fn parse_expr(&self, input: Span<'a>) -> Expr {
        if let Ok((input, val)) = digit1::<Span<'a>, ParseError>().parse(input) {
            Expr::Value(val.parse::<f32>().unwrap())
        } else if let Ok((input, id)) = alpha1::<Span<'a>, ParseError>(input) {
            Expr::Id(id.to_string())
        } else {
            Expr::BinOp(input.to_string())
        }
    }

    pub fn src(&self) -> Span<'a> {
        self.src
    }
}
