use crate::{
    primitives::{InfixOperation, RepData, Token, TokenType},
    traits::OptionStringify,
};

pub fn eval_statement(statement_token: &Token) {
    match statement_token.kind {
        TokenType::PRINT => println!("{}", statement_token.value.as_ref().option_as_string()),
        TokenType::BINOP(InfixOperation::Addition) => {
            if let Some(RepData::TWONUMBER(a, b)) = statement_token.value.as_ref() {
                print!("{}", a.clone() + b.clone());
            }
        }
        _ => (),
    }
}
