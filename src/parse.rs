use miette::{NamedSource, Result, SourceOffset, SourceSpan};

use crate::{
    error::{BadParenthesesError, UnexpectedTokenError},
    lexer::{Number, Operator, Token, TokenKind},
};

struct Expression {
    operands: (Number, Number),
    operator: Operator,
}

pub struct Parser<'a> {
    src: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str, tokens: Vec<Token>) -> Self {
        Self { src, tokens }
    }

    pub fn parse_as_rpn(self) -> Result<Vec<Token>> {
        let mut output_queue: Vec<Token> = vec![];
        let mut operator_stack: Vec<Token> = vec![];

        for token in self.tokens {
            match token.kind {
                TokenKind::Numeric(_) => output_queue.push(token),
                TokenKind::Op(ref o_1) => {
                    if operator_stack.last().is_none() {
                        operator_stack.push(token);
                        continue;
                    }

                    // Operator precedence:
                    // /
                    // *
                    // +, -
                    while operator_stack.last().is_some_and(|o_2| match o_2 {
                        Token {
                            kind: TokenKind::Op(o_2),
                            ..
                        } => o_2.has_greater_precedence_than(&o_1),
                        _ => panic!(""),
                    }) {
                        let o_2 = operator_stack.pop().unwrap();
                        output_queue.push(o_2);
                    }

                    operator_stack.push(token);
                }
                TokenKind::LeftParen => operator_stack.push(token),
                TokenKind::RightParen => {
                    if operator_stack.is_empty() {
                        Err(BadParenthesesError {
                            src: NamedSource::new("mathexpr", self.src.to_owned()),
                            err_span: {
                                let start = SourceOffset::from_location(self.src, 1, 1);
                                SourceSpan::new(start, 1)
                            },
                        })?;
                    }
                }
                _ => (),
            }
        }

        operator_stack.reverse();
        output_queue.append(&mut operator_stack);

        Ok(output_queue)
    }
}
