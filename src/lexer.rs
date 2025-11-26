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

use crate::Token;
use crate::math_lexing::MathLexer;
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
    Assign(String, Vec<Token>),
    Print(Vec<Token>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Value(f32),
    Id(String),
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

    pub fn lex(mut self) -> IResult<'a, Vec<Statement>> {
        let (_, statements) = self.lex_all()?;
        assert!(!statements.is_empty());
        Ok((Span::new(""), statements))
    }

    fn lex_all(mut self) -> IResult<'a, Vec<Statement>> {
        let (_, statements) = separated_list0(ws_tag(";"), is_not(";")).parse(self.src)?;
        let statements: Vec<Statement> = statements
            .iter()
            .map(|st| {
                let (_, res) = Lexer::lex_statement(*st).unwrap();
                res
            })
            .collect();

        Ok((Span::new(""), statements))
    }

    fn lex_statement(input: Span) -> IResult<Statement> {
        assert!(!input.is_empty());

        if let Ok((input, (id, val))) =
            separated_pair(alpha1::<Span, ParseError>, ws_tag("="), alphanumeric0).parse(input)
        {
            let val = val.fragment();
            let id = id.fragment().to_string();
            let (_, value_expr) = Lexer::lex_expr(Span::new(&val))?;
            let (_, tokens) = MathLexer::new(value_expr).lex()?;
            Ok((input, Statement::Assign(id, tokens)))
        } else {
            let (_, input) = rest(input)?;
            let (_, print_expr) = Lexer::lex_expr(input).unwrap();
            let (_, tokens) = MathLexer::new(print_expr).lex()?;
            Ok((input, Statement::Print(tokens)))
        }
    }

    // TODO: handle recursion.
    fn lex_expr(mut input: Span) -> IResult<Expr> {
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

    fn lex_op(&self, input: Span<'a>) -> BinOp {
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
