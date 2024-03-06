#[derive(Debug)]
pub struct ASTNode {
    next_node: Box<ASTNode>,
    left_child: TOKEN,
    right_child: TOKEN,
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
            TOKEN::PRINT => String::from("PRINT"),
            TOKEN::STRING(strung) => format!("STRING({})", strung),
            TOKEN::FUNCTION => String::from("FUNCTION"),
            TOKEN::RPAREN => String::from("RPAREN"),
            TOKEN::LPAREN => String::from("LPAREN"),
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
