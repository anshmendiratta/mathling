use crate::primitives::{ASTNode, Token, TokenType};

pub fn eval_statement(statement_token: &Token) {
    match statement_token.kind {
        TokenType::PRINT => println!("{}", statement_token.value.as_ref().unwrap()),
        _ => (),
    }
}

pub fn eval_syntax_tree(root_node: ASTNode) {
    let mut current_node: ASTNode = root_node;
    loop {
        let token_statement: &Token = &current_node.left_child;
        eval_statement(token_statement);

        if current_node.get_next_node().is_none() {
            break;
        }

        current_node = current_node.get_next_node().unwrap();
    }
}
