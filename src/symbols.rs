use std::collections::HashMap;

use crate::{lexer::Expr, math_lexing::MathLexer, Token};

pub struct SymbolTable {
    variables: HashMap<String, Vec<Token>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn lookup(&self, id: &str) -> Option<&Vec<Token>> {
        self.variables.get(id)
    }

    pub fn add(&mut self, id: &str, val: Vec<Token>) {
        self.variables.insert(id.to_owned(), val);
    }

    pub fn eval_symbol(&mut self, id: &str) -> Option<f32> {
        let symbol_expr = self.lookup(id);
        // if let
    }
}
