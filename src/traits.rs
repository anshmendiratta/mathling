use crate::primitives::{Token, TokenType};

pub trait Stringify<T>
where
    T: std::fmt::Debug,
{
    fn as_string(&self) -> String;
}

impl Stringify<Token> for Token {
    fn as_string(&self) -> String {
        match self.kind {
            TokenType::PRINT => String::from("PRINT"),
            TokenType::STRING => format!("STRING({:?})", self.value.clone().unwrap()),
            TokenType::FUNCTION => String::from("FUNCTION"),
            TokenType::RPAREN => String::from("RPAREN"),
            TokenType::LPAREN => String::from("LPAREN"),
            TokenType::RETURN => format!("RETURN({:?})", self.value.clone().unwrap()),
            TokenType::NULL => String::from("NULL"),
        }
    }
}

impl From<String> for Token {
    fn from(val: String) -> Self {
        match &val[..] {
            "PRINT" => TokenType::PRINT.into(),
            "STRING" => TokenType::STRING.into(),
            "FUNCTION" => TokenType::FUNCTION.into(),
            "RPAREN" => TokenType::RPAREN.into(),
            "RLAREN" => TokenType::LPAREN.into(),
            "RETURN" => TokenType::RETURN.into(),
            _ => TokenType::NULL.into(),
        }
    }
}

pub trait OptionStringify<T>
where
    T: std::fmt::Debug,
{
    fn option_as_string(&self) -> String;
}

impl<T> OptionStringify<T> for Option<T>
where
    T: std::fmt::Debug,
{
    fn option_as_string(&self) -> String {
        if self.is_none() {
            return "".to_string();
        }

        let formatted_string: String = format!("{:?}", self);
        let maybe_extracted_string: String = formatted_string
            .strip_prefix("Some(")
            .expect("String empty?")
            .strip_suffix(')')
            .expect("Second string empty?")
            .to_string();

        if maybe_extracted_string.starts_with('"') && maybe_extracted_string.ends_with('"') {
            return maybe_extracted_string.trim_matches('"').to_string();
        }

        maybe_extracted_string
    }
}

#[cfg(test)]
mod test {
    use crate::{
        primitives::{Token, TokenType},
        traits::OptionStringify,
    };

    #[test]
    fn check_extraction_of_option() {
        let option_type_1: Option<usize> = Some(1);
        let option_type_2: Option<&str> = Some("WOWIE");
        let option_type_3: Option<usize> = None;

        assert_eq!("1", option_type_1.option_as_string());
        assert_eq!("WOWIE", option_type_2.option_as_string());
        assert_eq!("", option_type_3.option_as_string());
    }

    #[test]
    fn check_token_as_string() {
        let check_1: Token = TokenType::PRINT.into();
        let check_2: Token = TokenType::RPAREN.into();
        let check_3: Token = TokenType::STRING.into();

        assert_eq!(r#"PRINT"#, check_1.as_string());
        assert_eq!("RPAREN", check_2.as_string());
        assert_eq!(r#"Some(STRING(""))"#, check_3.as_string());
    }
}
