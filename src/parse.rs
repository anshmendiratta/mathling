use crate::{
    IResult, Span, Token, TokenType, error::ParseError, lexer::Statement, math_lexing::MathLexer,
    symbols::SymbolTable,
};

pub struct Parser {
    statements: Vec<Statement>,
}

impl Parser {
    pub fn new(tokens: Vec<Statement>) -> Self {
        Self { statements: tokens }
    }

    pub fn parse<'a>(mut self) -> IResult<'a, (Vec<Token>, SymbolTable)> {
        // Checks for at most one print.
        assert!(
            self.statements
                .iter()
                .filter(|s| if let Statement::Print(_) = s {
                    return true;
                } else {
                    return false;
                })
                .count()
                <= 1
        );

        let mut symbol_table = SymbolTable::new();
        for statement in &self.statements {
            if let Statement::Assign(id, expr) = statement {
                symbol_table.add(id, expr.clone());
            } else if let Statement::Print(print) = statement {
                // Should only run once.
                let (_, rpn) = Parser::parse_into_rpn(print.to_vec())?;
                return Ok((Span::new(""), (rpn, symbol_table)));
            }
        }

        unreachable!()
    }

    /// Implements shunting yard: https://en.wikipedia.org/wiki/Shunting_yard_algorithm#The_algorithm_in_detail
    pub fn parse_into_rpn<'a>(tokens: Vec<Token>) -> IResult<'a, Vec<Token>> {
        let mut output_queue: Vec<Token> = vec![];
        let mut operator_stack: Vec<Token> = vec![];

        for token in tokens {
            match token.token_type {
                TokenType::Fp(_) | TokenType::Id(_) => output_queue.push(token),
                TokenType::BinOp(ref o_1) => {
                    while operator_stack
                        .last()
                        .is_some_and(|o_2| match &o_2.token_type {
                            TokenType::BinOp(o_2) => o_2.has_greater_precedence_than(o_1),
                            TokenType::LeftParen => false,
                            _ => panic!(""),
                        })
                    {
                        let o_2 = operator_stack.pop().unwrap();
                        output_queue.push(o_2);
                    }

                    operator_stack.push(token);
                }
                TokenType::LeftParen => operator_stack.push(token),
                TokenType::RightParen => {
                    while operator_stack
                        .last()
                        .is_some_and(|t| t.token_type != TokenType::LeftParen)
                    {
                        let last_op = operator_stack.pop().unwrap();
                        output_queue.push(last_op);
                    }
                    if operator_stack.is_empty() {
                        let op_msg = format!("{:?}", operator_stack);
                        return Err(nom::Err::Error(ParseError::new(
                            // Span::new(&op_msg),
                            Span::new(""),
                            String::from("Operator stack not empty."),
                        )));
                    }

                    operator_stack.pop();
                }
            }
        }

        while !operator_stack.is_empty() {
            if operator_stack.first().is_some_and(|t| {
                t.token_type == TokenType::LeftParen || t.token_type == TokenType::RightParen
            }) {
                return Err(nom::Err::Error(ParseError::new(
                    // Span::new(&format!("{:?}", operator_stack)),
                    "".into(),
                    String::from("Unclosed parenthesis."),
                )));
            }
            let last_op = operator_stack.pop().unwrap();
            output_queue.push(last_op);
        }

        operator_stack.reverse();
        output_queue.append(&mut operator_stack);

        Ok((Span::new(""), output_queue))
    }

    fn eval_rpn(mut rpn: Vec<Token>) -> IResult<'static, f32> {
        let mut result = 0.;

        while !rpn.is_empty() {}

        Ok((Span::new(""), result))
    }
}
