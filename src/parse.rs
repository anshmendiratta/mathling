use crate::lexer::{Number, Operator, Token};

struct Expression {
    operands: (Number, Number),
    operator: Operator,
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse_as_rpn(&self) -> Vec<Token> {
        let mut output_queue: Vec<Token> = vec![];
        let mut operator_stack: Vec<Operator> = vec![];

        for token in self.tokens.clone() {
            match token {
                Token::Numeric(_) => output_queue.push(token),
                Token::Op(o_1) => {
                    if operator_stack.last().is_none() {
                        operator_stack.push(o_1);
                        continue;
                    }

                    // Operator precedence:
                    // /
                    // *
                    // +, -
                    while operator_stack
                        .last()
                        .is_some_and(|o_2| o_2.has_greater_precedence_than(o_1.clone()))
                    {
                        let o_2 = operator_stack.pop().unwrap();
                        output_queue.push(Token::Op(o_2));
                    }

                    operator_stack.push(o_1);
                }
                _ => (),
            }
        }

        operator_stack.reverse();
        let mut operator_stack_as_tokens: Vec<Token> = operator_stack
            .iter()
            .map(|op| Token::Op(op.clone()))
            .collect();
        output_queue.append(&mut operator_stack_as_tokens);

        output_queue
    }
}
