#![allow(unused_imports)]

use std::ops::Deref;
use std::{collections::HashMap, fmt::Write, ops::Range};

use nom::bytes::complete::is_not;
use nom::multi::separated_list0;
use nom::{
    Parser,
    branch::alt,
    bytes::is_a,
    character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1},
    combinator::{recognize, rest},
    multi::many0,
    sequence::{delimited, separated_pair},
};

use crate::{IResult, Span, error::ParseError, util::ws_tag};

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &BinOp::Plus => f.write_str("+"),
            &BinOp::Minus => f.write_str("-"),
            &BinOp::Times => f.write_str("*"),
            &BinOp::Divide => f.write_str("/"),
            &BinOp::Equal => f.write_str("="),
        }
    }
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

#[derive(Debug, Clone)]
pub enum Statement {
    Assign(String, Expr),
    Print(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Value(f32),
    Id(String),
    // BinOp(Box<(Expr, BinOp, Expr)>),
    BinOp(String),
}

pub struct Lexer<'a> {
    src: Span<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(expr_as_string: &'a str) -> Self {
        assert!(!expr_as_string.is_empty());
        Self {
            src: Span::new(expr_as_string),
        }
    }

    pub fn lex(&'_ mut self) -> IResult<'_, Vec<Statement>> {
        let (_, statements) = self.parse_all()?;
        assert!(!statements.is_empty());
        Ok((Span::new(""), statements))
    }

    fn parse_all(&'_ mut self) -> IResult<'_, Vec<Statement>> {
        let (_, statements) = separated_list0(ws_tag(";"), is_not(";")).parse(self.src)?;
        let statements: Vec<Statement> = statements.iter().map(|st| {
            let (_, res) = Lexer::parse_statement(*st).unwrap();
            res
            }
                ).collect();

        Ok((Span::new(""), statements))
    }

    fn parse_statement(input: Span) -> IResult< Statement> {
        assert!(!input.is_empty());

        if let Ok((input, (id, val))) =
            separated_pair(alpha1::<Span, ParseError>, ws_tag("="), alphanumeric0).parse(input)
        {
            let val = val.fragment();
            let id = id.fragment().to_string();
            let (_, value_expr) = Lexer::parse_expr(Span::new(&val))?;
            Ok((input, Statement::Assign(id, value_expr)))
        } else {
            let (_, input) = rest(input)?;
            let (_, print_expr) = Lexer::parse_expr(input).unwrap();
            Ok((input, Statement::Print(print_expr)))
        }
    }

    // TODO: handle recursion.
    fn parse_expr(mut input: Span) -> IResult< Expr> {
        if let Ok((input, val)) = digit1::<Span, ParseError>(input)
            && input.is_empty()
        {
            Ok((Span::new(""), Expr::Value(val.parse::<f32>().unwrap())))
        } else if let Ok((input, id)) = alpha1::<Span, ParseError>(input)
            && input.is_empty()
        {
            Ok((Span::new(""), Expr::Id(id.to_string())))
        } else {
            // Assume binary operation.
            Ok((
                Span::new(""),
                // Expr::BinOp(Box::new((l_expr, bin_op, r_expr))),
                Expr::BinOp(input.to_string()),
            ))
        }
    }

    fn parse_op(&self, input: Span<'a>) -> BinOp {
        match *input.fragment() {
            "+" => BinOp::Plus,
            "-" => BinOp::Minus,
            "*" => BinOp::Times,
            "/" => BinOp::Divide,
            "=" => BinOp::Equal,
            _ => unreachable!("weird op"),
        }
    }

    pub fn src(&self) -> Span<'a> {
        self.src
    }
}
