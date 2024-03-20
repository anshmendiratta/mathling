use crate::primitives::{TOKEN, TOKENTYPE};

pub trait Stringify<T>
where
    T: std::fmt::Debug,
{
    fn as_string(&self) -> String;
}

impl Stringify<TOKEN> for TOKEN {
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
            .trim_start_matches("Some(")
            .trim_end_matches(')')
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
        primitives::{TOKEN, TOKENTYPE},
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
        let check_1: TOKEN = TOKENTYPE::PRINT.into();
        let check_2: TOKEN = TOKENTYPE::RPAREN.into();
        let check_3: TOKEN = TOKENTYPE::STRING.into();

        assert_eq!(r#"PRINT"#, check_1.as_string());
        assert_eq!("RPAREN", check_2.as_string());
        assert_eq!(r#"Some(STRING(""))"#, check_3.as_string());
    }
}
