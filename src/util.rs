use nom::{
    bytes::tag,
    character::{complete::one_of, multispace0},
    combinator::{map, opt, recognize},
    multi::many1,
    sequence::{delimited, preceded},
    Parser,
};

use crate::{
    lexer::{Number, Operator},
    IResult,
};

pub fn find_decimal(stream: &str) -> IResult<&str> {
    recognize(many1(one_of("0123456789"))).parse(stream)
}

pub fn find_fp(stream: &str) -> IResult<Number> {
    let (rest, integral) = find_decimal(stream)?;
    let (_, fractional) = opt(preceded(tag("."), find_decimal)).parse(rest)?;
    let fp_as_string = match fractional {
        Some(fract) => integral.to_string() + "." + &fract,
        None => integral.to_string(),
    };
    let fp = fp_as_string.parse::<f32>().unwrap();

    Ok((&rest, Number(fp)))
}

pub fn find_op(stream: &str) -> IResult<Operator> {
    let ws_one_of = delimited(multispace0(), one_of("+-*/"), multispace0());
    map(ws_one_of, |op| match op {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        '*' => Operator::Asterisk,
        '/' => Operator::Slash,
        _ => unreachable!(),
    })
    .parse(stream)
}

pub fn find_assign(stream: &str) -> IResult<&str> {
    delimited(multispace0(), tag("="), multispace0()).parse(stream)
}

// pub fn ws_parser<T: Input>(
//     parser: impl Parser<T, Error = nom::error::Error<&'static str>, Output = char>,
// ) -> impl Parser<T, Error = nom::error::Error<&'static str>, Output = char>
// where
//     <T as Input>::Item: AsChar,
//     nom::error::Error<&'static str>: ParseError<T>,
// {
//     delimited(multispace0(), parser, multispace0())
// }
