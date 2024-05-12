use crate::primitives::{Token, TokenType};

pub fn eval_statement(statement_token: &Token) {
    match statement_token.kind {
        TokenType::PRINT => print!("{}", statement_token.value.as_ref().unwrap()),
        _ => (),
    }
}
