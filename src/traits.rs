use crate::primitives::{Number, RepData, Token, TokenType};

pub trait Stringify<T>
where
    T: std::fmt::Debug,
{
    fn as_string(&self) -> String;
}

impl Stringify<Token> for Token {
    fn as_string(&self) -> String {
        match &self.kind {
            TokenType::PRINT => String::from("PRINT"),
            TokenType::STRING => format!("STRING({:?})", self.value.as_ref().unwrap()),
            TokenType::FUNCTION => String::from("FUNCTION"),
            TokenType::RPAREN => String::from("RPAREN"),
            TokenType::LPAREN => String::from("LPAREN"),
            TokenType::RETURN => format!("RETURN({:?})", self.value.as_ref().unwrap()),
            TokenType::NULL => String::from("NULL"),
            TokenType::BINOP(op) => format!("BINOP({:?})", op),
            TokenType::NUMBER => String::from("NUMBER"),
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

        let open_brackets_indices: Vec<(usize, &str)> =
            formatted_string.match_indices('(').collect();
        let last_open_bracket_index: usize =
            open_brackets_indices[open_brackets_indices.len().saturating_sub(1)].0;
        let close_brackets_indices: Vec<(usize, &str)> =
            formatted_string.match_indices(')').collect();
        let first_close_bracket_index: usize = close_brackets_indices[0].0;

        let maybe_extracted_string: &str =
            &formatted_string[last_open_bracket_index + 1..first_close_bracket_index];

        if maybe_extracted_string.starts_with('"') && maybe_extracted_string.ends_with('"') {
            return maybe_extracted_string.trim_matches('"').to_string();
        }

        maybe_extracted_string.to_string()
    }
}

impl std::fmt::Display for RepData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepData::STRING(s) => writeln!(f, "{}", s)?,
            RepData::NUMBER(n) => match n {
                Number::UINT(ui) => writeln!(f, "{}", ui)?,
                Number::IINT(ii) => writeln!(f, "{}", ii)?,
            },
            RepData::TWONUMBER(n, m) => match (n, m) {
                (Number::UINT(ui_1), Number::UINT(ui_2)) => writeln!(f, "{}, {}", ui_1, ui_2)?,
                (Number::IINT(ii_1), Number::IINT(ii_2)) => writeln!(f, "{}, {}", ii_1, ii_2)?,
                (Number::UINT(ui), Number::IINT(ii)) => writeln!(f, "{}, {}", ui, ii)?,
                (Number::IINT(ii), Number::UINT(ui)) => writeln!(f, "{}, {}", ii, ui)?,
            },
        };

        Ok(())
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::UINT(ui) => writeln!(f, "{}", ui)?,
            Number::IINT(ii) => writeln!(f, "{}", ii)?,
        };

        Ok(())
    }
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::UINT(ui_1), Number::UINT(ui_2)) => Number::UINT(ui_1 + ui_2),
            (Number::IINT(ii_1), Number::IINT(ii_2)) => Number::IINT(ii_1 + ii_2),
            _ => panic!("Inconsistent data types"),
        }
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
