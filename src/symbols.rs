use std::collections::HashMap;

use crate::{Token, lexer::Expr, math_lexing::MathLexer};

pub struct SymbolTable {
    variables: HashMap<String, f32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn lookup(&self, id: &str) -> Option<&f32> {
        self.variables.get(id)
    }

    pub fn add(&mut self, id: &str, val: f32) {
        self.variables.insert(id.to_owned(), val);
    }

    // pub fn eval_symbol(&mut self, id: &str) -> Option<f32> {
    //     let symbol_expr = self.lookup(id);
    //     // if let
    // }
}
