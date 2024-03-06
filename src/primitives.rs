#[derive(Debug)]
pub struct ASTNode {
    next_node: Box<ASTNode>,
    left_child: TOKEN,
    right_child: TOKEN,
}

impl std::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {
            let left_child: ASTNode = self.left_child;
            let right_child: ASTNode = self.right_child;

            if () {}
        }
    }
}

#[derive(Debug)]
pub enum RETURNABLE {
    STRING(String),
    UINT(usize),
    IINT(isize),
}

#[derive(Debug)]
pub enum TOKEN {
    PRINT,
    STRING(String),
    FUNCTION,
    RPAREN,
    LPAREN,
    RETURN(RETURNABLE),
}

impl TOKEN {
    fn as_string(&self) -> String {
        match self {
            TOKEN::PRINT => "PRINT".to_owned(),
            TOKEN::STRING(strung) => format!("STRING({})", strung),
            TOKEN::FUNCTION => "FUNCTION".to_owned(),
            TOKEN::RPAREN => "RPAREN".to_owned(),
            TOKEN::LPAREN => "LPAREN".to_owned(),
            TOKEN::RETURN(returnable) => format!("RETURN({:?})", returnable),
        }
    }
}

mod token_methods {
    use super::{RETURNABLE, TOKEN};

    fn match_token_string(token_string: String) -> Option<TOKEN> {
        match &token_string[..] {
            "PRINT" => Some(TOKEN::PRINT),
            "STRING" => Some(TOKEN::STRING("".to_string())),
            "FUNCTION" => Some(TOKEN::FUNCTION),
            "RPAREN" => Some(TOKEN::RPAREN),
            "RLAREN" => Some(TOKEN::LPAREN),
            "RETURN" => Some(TOKEN::RETURN(RETURNABLE::STRING("".to_string()))),
            _ => None,
        }
    }
}
