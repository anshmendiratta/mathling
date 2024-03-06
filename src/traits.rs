use crate::primitives::TOKEN;

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
