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

    fn parse() -> Vec<Token> {
        vec![]
    }
}
