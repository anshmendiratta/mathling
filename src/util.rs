use std::error::Error;

use nom::{
    branch::alt,
    bytes::{complete::is_not, tag},
    character::{
        char,
        complete::{alpha0, alphanumeric1, one_of, space0},
        multispace0,
        streaming::{alpha1, alphanumeric0},
    },
    combinator::{map, opt, recognize},
    multi::many1,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    Parser,
};

use crate::{
    lexer::{BinOp, Expr, Statement},
    IResult, Span,
};

// SINGLETONS.

pub fn find_decimal(input: Span) -> IResult<Span> {
    recognize(many1(one_of("0123456789"))).parse(input)
}

pub fn find_fp(stream: Span) -> IResult<Expr> {
    let (rest, integral) = find_decimal(stream)?;
    let (_, fractional) = opt(preceded(tag("."), find_decimal)).parse(rest)?;
    let fp_as_string = match fractional {
        Some(fract) => integral.to_string() + "." + &fract,
        None => integral.to_string(),
    };
    let fp = fp_as_string.parse::<f32>().unwrap();

    Ok((rest, Expr::Value(fp)))
}

pub fn find_identifier(input: Span) -> IResult<Expr> {
    map(
        recognize(pair(many1(alpha1), alt((tag("_"), alpha0)))),
        |(rest, id)| (rest, Expr::Id(id)),
    )
    .parse(input)?
}

pub fn find_identifier_expr(input: Span) -> IResult<Span> {
    let (input, id) = find_identifier(input)?;
    let (expr, _) = find_equal(input)?;

    Ok((Span::new(""), expr))
}

pub fn find_inside_expr(input: Span) -> IResult<Span> {
    delimited(tag("("), is_not(")"), tag(")")).parse(input)
}

pub fn find_binop_expr(input: Span) -> IResult<Expr> {
    let mut ws_op = delimited(space0, one_of("+-*/"), space0);
    let (input, left) = is_not("+-*/ ").parse(input)?;
    let (right, op) = ws_op.parse(input)?;

    if !input.contains("(") && !input.contains(")") {
        let (rest_l, id_l) = alt((find_identifier, find_fp)).parse(left)?;
        let (rest_r, id_r) = find_identifier(right)?;
        Ok((
            Span::new(""),
            Expr::BinOp(op, Box::new((Expr::Value(()), Expr::Value()))),
        ));
    }

    let mut ws_op = delimited(space0, one_of("+-*/"), space0);
    let op = match op {
        '+' => BinOp::Plus,
        '-' => BinOp::Minus,
        '*' => BinOp::Times,
        '/' => BinOp::Divide,
    };

    while (left.contains("(") && left.contains(")")) {
        (_, left) = find_binop_expr(left)?;
    }
    while (right.contains("(") && right.contains(")")) {
        right = find_binop_expr(right);
    }

    Ok((Span::new(""), Expr::BinOp(op, Box::new())))
}

// STATEMENTS.

pub fn find_assign(input: Span) -> IResult<Statement> {
    let (input, id) = find_identifier(input)?;
    let (_, mut expr) = find_identifier_expr(input)?;
    while expr.contains("(") && expr.contains(")") {
        let (_, e) = find_inside_expr(expr)?;
        expr = e;
    }

    // let assignment = Statement::Assign((), ())
    Ok((Span::new(""), Statement::Print(Expr::Value(2.0))))
}

/// Returns the span after the equal.
pub fn find_equal(input: Span) -> IResult<Span> {
    delimited(space0, tag("="), space0).parse(input)
}

pub fn ws<'a, F, O>(parser: F) -> impl Parser<Span<'a>>
where
    F: Parser<Span<'a>>,
{
    delimited(space0, parser, space0)
}
