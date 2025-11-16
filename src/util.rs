use std::error::Error;

use nom::{
    AsChar, Input, Parser,
    branch::{Choice, alt},
    bytes::complete::{is_a, is_not, tag},
    character::{
        char,
        complete::{alpha0, alphanumeric1, one_of, space0},
        multispace0,
        streaming::{alpha1, alphanumeric0},
    },
    combinator::{map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};

use crate::{
    IResult, Span,
    error::{self, ParseError},
    lexer::{BinOp, Expr, Statement},
};

pub fn ws_tag(input: &str) -> impl Parser<Span<'_>, Output = Span<'_>, Error = ParseError<'_>> {
    delimited(space0, tag(input), space0)
}
