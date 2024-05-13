use crate::primitives::{InfixOperation, RepData, Token, TokenType};

pub fn eval_statement(statement_token: &Token) {
    match statement_token.kind {
        TokenType::PRINT => print!("{}", statement_token.value.as_ref().unwrap()),
        TokenType::BINOP(InfixOperation::Addition) => {
            if let RepData::TWONUMBER(a, b) = statement_token.value.as_ref().unwrap() {
                print!("{}", a.clone() + b.clone());
            }
        }
        _ => (),
    }
}
