use crate::primitives::{Token, TokenType};

pub fn eval_statement(statement_token: Token) {
    match statement_token.kind {
        TokenType::PRINT => println!("{}", statement_token.value.unwrap()),
        _ => (),
    }
}
