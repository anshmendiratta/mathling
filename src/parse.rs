use miette::{NamedSource, Result, SourceOffset, SourceSpan};

use crate::{
    error::BadParenthesesError,
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

    /// Implements shunting yard: https://en.wikipedia.org/wiki/Shunting_yard_algorithm#The_algorithm_in_detail
    pub fn parse_into_rpn(self) -> Result<Vec<Token>> {
        let mut output_queue: Vec<Token> = vec![];
        let mut operator_stack: Vec<Token> = vec![];

        for token in self.tokens {
            match token.kind {
                TokenKind::Numeric(_) => output_queue.push(token),
                TokenKind::Op(ref o_1) => {
                    while operator_stack.last().is_some_and(|o_2| match o_2 {
                        Token {
                            kind: TokenKind::Op(o_2),
                            ..
                        } => o_2.has_greater_precedence_than(o_1),
                        Token {
                            kind: TokenKind::LeftParen,
                            ..
                        } => false,
                        _ => panic!(""),
                    }) {
                        let o_2 = operator_stack.pop().unwrap();
                        output_queue.push(o_2);
                    }

                    operator_stack.push(token);
                }
                TokenKind::LeftParen => operator_stack.push(token),
                TokenKind::RightParen => {
                    while operator_stack
                        .last()
                        .is_some_and(|t| t.kind != TokenKind::LeftParen)
                    {
                        let last_op = operator_stack.pop().unwrap();
                        output_queue.push(last_op);
                    }
                    if operator_stack.is_empty() {
                        Err(BadParenthesesError {
                            src: NamedSource::new("mathexpr", self.src.to_owned()),
                            err_span: {
                                let start = SourceOffset::from_location(self.src, 1, 1);
                                SourceSpan::new(start, 1)
                            },
                        })?;
                    }

                    operator_stack.pop();
                }
                _ => {}
            }
        }

        while !operator_stack.is_empty() {
            if operator_stack
                .first()
                .is_some_and(|t| t.kind == TokenKind::LeftParen || t.kind == TokenKind::RightParen)
            {
                Err(BadParenthesesError {
                    src: NamedSource::new("mathexpr", self.src.to_owned()),
                    err_span: {
                        let start = SourceOffset::from_location(self.src, 1, 1);
                        SourceSpan::new(start, 1)
                    },
                })?;
            }
            let last_op = operator_stack.pop().unwrap();
            output_queue.push(last_op);
        }

        operator_stack.reverse();
        output_queue.append(&mut operator_stack);

        Ok(output_queue)
    }
}
