/// `left_child`: `TOKEN`
/// `right_child`: equals `next_node` that is a recursive
#[derive(Debug, Clone)]
pub struct ASTNode {
    pub left_child: TOKEN,
    pub next_node: Option<Box<ASTNode>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TOKEN {
    pub kind: TOKENTYPE,
    pub value: Option<REPDATA>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TOKENTYPE {
    PRINT,
    STRING,
    FUNCTION,
    RPAREN,
    LPAREN,
    RETURN,
    NULL,
}

#[derive(Debug, Clone, PartialEq)]
pub enum REPDATA {
    STRING(String),
    UINT(usize),
    IINT(isize),
}

impl TOKEN {
    pub fn as_string(&self) -> String {
        match self.kind {
            TOKENTYPE::PRINT => "PRINT".to_owned(),
            TOKENTYPE::STRING => format!("{:?}", self.value),
            TOKENTYPE::FUNCTION => "FUNCTION".to_owned(),
            TOKENTYPE::RPAREN => "RPAREN".to_owned(),
            TOKENTYPE::LPAREN => "LPAREN".to_owned(),
            TOKENTYPE::RETURN => format!("{:?}", self.value),
            TOKENTYPE::NULL => "NULL".to_owned(),
        }
    }
}

impl From<TOKENTYPE> for TOKEN {
    fn from(value: TOKENTYPE) -> Self {
        let self_default_value = match &value {
            TOKENTYPE::PRINT => Some(REPDATA::STRING("".to_string())),
            TOKENTYPE::STRING => Some(REPDATA::STRING("".to_string())),
            _ => None
        };

        TOKEN {
            kind: value,
            value: self_default_value,
        }
    }
}

#[cfg(test)]
mod tests {}
