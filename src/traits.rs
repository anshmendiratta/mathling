use crate::primitives::{ASTNode, TOKEN};

pub trait Stringify {
    fn as_string(&self) -> Vec<String>;
}

impl Stringify for Vec<TOKEN> {
    fn as_string(&self) -> Vec<String> {
        let mut string_tokens: Vec<String> = Vec::new();
        for token in self {
            match token {
                TOKEN::PRINT => string_tokens.push(String::from("PRINT")),
                TOKEN::STRING(strung) => string_tokens.push(format!("STRING({})", strung)),
                TOKEN::FUNCTION => string_tokens.push(String::from("FUNCTION")),
                TOKEN::RPAREN => string_tokens.push(String::from("RPAREN")),
                TOKEN::LPAREN => string_tokens.push(String::from("LPAREN")),
                TOKEN::RETURN(returnable) => {
                    string_tokens.push(format!("RETURN({:?})", returnable))
                }
            }
        }

        string_tokens
    }
}

impl std::fmt::Display for ASTNode {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left_child: TOKEN = self.left_child.clone();
        let mut right_child: Option<Box<ASTNode>> = self.next_node.clone();

        loop {
            match right_child.clone() {
                Some(further_node) => {
                    println!(
                        "{}\n | \t \\ \t  {} \t {}",
                        left_child.as_string(),
                        further_node.left_child.as_string(),
                        *further_node
                    );
                    right_child = Some(further_node.clone());
                }
                _ => break,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_ast_nodes() {
        let child_node: ASTNode = ASTNode {
            left_child: TOKEN::PRINT,
            next_node: None,
        };

        let node1: ASTNode = ASTNode {
            left_child: TOKEN::PRINT,
            next_node: Some(Box::new(child_node)),
        };

        println!("{}", node1);
    }
}
