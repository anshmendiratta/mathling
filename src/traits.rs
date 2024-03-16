use crate::primitives::{TOKEN, TOKENTYPE};

pub trait Stringify {
    fn as_string(&self) -> String;
}

impl Stringify for TOKEN {
    fn as_string(&self) -> String {
        match self.kind {
            TOKENTYPE::PRINT => String::from("PRINT"),
            TOKENTYPE::STRING => format!("STRING({:?})", self.value.clone().unwrap()),
            TOKENTYPE::FUNCTION => String::from("FUNCTION"),
            TOKENTYPE::RPAREN => String::from("RPAREN"),
            TOKENTYPE::LPAREN => String::from("LPAREN"),
            TOKENTYPE::RETURN => format!("RETURN({:?})", self.value.clone().unwrap()),
            TOKENTYPE::NULL => String::from("NULL"),
        }
    }
}

impl From<String> for TOKEN {
    fn from(val: String) -> Self {
        match &val[..] {
            "PRINT" => TOKENTYPE::PRINT.into(),
            "STRING" => TOKENTYPE::STRING.into(),
            "FUNCTION" => TOKENTYPE::FUNCTION.into(),
            "RPAREN" => TOKENTYPE::RPAREN.into(),
            "RLAREN" => TOKENTYPE::LPAREN.into(),
            "RETURN" => TOKENTYPE::RETURN.into(),
            _ => TOKENTYPE::NULL.into(),
        }
    }
}
