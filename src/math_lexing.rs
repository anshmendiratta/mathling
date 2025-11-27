use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::{alpha0, alpha1, digit1},
    combinator::{map_res, opt, recognize},
    error::Error,
    multi::many0,
    sequence::preceded,
    Parser,
};

use crate::{
    error::ParseError,
    lexer::{BinOp, Expr},
    util::ws_tag,
    IResult, Span, Token,
};

pub struct MathLexer {
    input: Expr,
}

impl MathLexer {
    pub fn new(input: Expr) -> Self {
        Self { input }
    }

    pub fn lex(mut self) -> IResult<'static, Vec<Token>> {
        match self.input {
            Expr::Value(v) => {
                return Ok((
                    Span::new(""),
                    vec![Token {
                        token_type: crate::TokenType::Fp(v),
                        location_col: None,
                    }],
                ));
            }
            Expr::Id(id) => {
                return Ok((
                    Span::new(""),
                    vec![Token {
                        token_type: crate::TokenType::Id(id),
                        location_col: None,
                    }],
                ));
            }
            Expr::BinOp(bin_op) => MathLexer::lex_bin_op(bin_op),
        }
    }

    fn lex_bin_op(input: String) -> IResult<'static, Vec<Token>> {
        let mut tokens = Vec::new();
        let mut rest = Span::new(&input);
        while !rest.fragment().is_empty() {
            // Number.
            match MathLexer::lex_fp(rest) {
                Ok((input, fp)) => {
                    rest = input;
                    tokens.push(fp);
                }
                _ => (),
            }
            // Identifier.
            match alpha1::<Span, ParseError>.parse(rest) {
                Ok((input, id)) => {
                    rest = input;
                    let val = Token {
                        token_type: crate::TokenType::Id(id.to_string()),
                        location_col: Some(input.location_offset() - id.to_string().len()),
                    };
                    tokens.push(val);
                }
                _ => (),
            }
            // Operator.
            match map_res(
                alt((ws_tag("+"), ws_tag("-"), ws_tag("*"), ws_tag("/"))),
                |op| match *op {
                    "+" => Ok::<BinOp, ParseError>(BinOp::Plus),
                    "-" => Ok(BinOp::Minus),
                    "*" => Ok(BinOp::Times),
                    "/" => Ok(BinOp::Divide),
                    _ => Err(ParseError::new(
                        Span::new(""), // TODO: get real input.
                        "found invalid operator".to_owned(),
                    )),
                },
            )
            .parse(rest)
            {
                Ok((input, op)) => {
                    rest = input;
                    let val = Token {
                        token_type: crate::TokenType::BinOp(op.clone()),
                        location_col: Some(input.location_offset() - op.to_string().len()),
                    };
                    tokens.push(val);
                }
                _ => (),
            }
            match many0(ws_tag("(")).parse(rest) {
                Ok((input, captures)) => {
                    rest = input;
                    let val = Token {
                        token_type: crate::TokenType::LeftParen,
                        location_col: Some(
                            input.location_offset()
                                - captures.first().unwrap_or(&Span::new("")).to_string().len(),
                        ),
                    };
                    for _ in 0..captures.len() {
                        tokens.push(val.clone());
                    }
                }
                _ => (),
            }
            match many0(ws_tag(")")).parse(rest) {
                Ok((input, captures)) => {
                    rest = input;
                    let val = Token {
                        token_type: crate::TokenType::RightParen,
                        location_col: Some(
                            input.location_offset()
                                - captures.first().unwrap_or(&Span::new("")).to_string().len(),
                        ),
                    };
                    for _ in 0..captures.len() {
                        tokens.push(val.clone());
                    }
                }
                _ => (),
            }
        }

        Ok((Span::new(""), tokens))
    }

    fn lex_fp(input: Span) -> IResult<Token> {
        let (rest, integral) = digit1.parse(input)?;
        let (rest, fractional) =
            opt(preceded(ws_tag("."), digit1::<Span, ParseError>)).parse(rest)?;
        let fp = match fractional {
            Some(fractional) => [integral.fragment(), *fractional].join("."),
            None => integral.fragment().to_string(),
        };
        let fp = fp.parse::<f32>().unwrap();

        Ok((
            rest,
            Token {
                token_type: crate::TokenType::Fp(fp),
                location_col: Some(rest.location_offset() - fp.to_string().len()),
            },
        ))
    }
}
