use crate::primitives::{REPDATA, TOKEN, TOKENTYPE};

pub fn tokenize(parse_string: String) -> Vec<TOKEN> {
    let mut read_buffer: Vec<&str> = Vec::new(); // Maximal munch
    let mut tokens: Vec<TOKEN> = Vec::new();
    let paren_scope: usize = 0;

    // let parse_string_lines: Vec<&str> = parse_string.split('\n');
    let parse_string_scopes = parse_string.split('(');

    for character in parse_string.chars() {
        if character == '(' {
            let _ = paren_scope.checked_add(1);
        } else if character == ')' {
            let _ = paren_scope.checked_sub(1);
        }
    }

    for line in parse_string_scopes {
        for keyword in line.split(')') {
            read_buffer.push(keyword.trim());
        }
    }

    for token_candidate in read_buffer {
        let mut token_buffer: Vec<char> = Vec::new();
        for character in token_candidate.chars() {
            token_buffer.push(character);

            match match_token_buffer(token_buffer.clone(), true) {
                Some(matched_token) => tokens.push(matched_token),
                _ => continue,
            }
        }
    }

    // dbg!(paren_scope);
    tokens
}

pub fn match_token_buffer(token_buffer: Vec<char>, read_from_source: bool) -> Option<TOKEN> {
    let token_buffer_as_string: &str = &token_buffer.iter().collect::<String>();
    match token_buffer_as_string {
        "print" => Some(TOKENTYPE::PRINT.into()),
        "(" => Some(TOKENTYPE::LPAREN.into()),
        ")" => Some(TOKENTYPE::RPAREN.into()),
        _ => match read_from_source {
            true => {
                let starts_with_apos: bool = token_buffer_as_string.starts_with('\'');
                let ends_with_apos: bool = token_buffer_as_string.ends_with('\'');
                if !(starts_with_apos && ends_with_apos && token_buffer_as_string.len() >= 2) {
                    return None;
                }
                let string_close_paren_index: usize = token_buffer_as_string.len() - 1;
                let token_value: REPDATA = REPDATA::STRING(
                    token_buffer_as_string[1..string_close_paren_index].to_string(),
                );
                let token: TOKEN = TOKEN {
                    kind: TOKENTYPE::STRING,
                    value: Some(token_value),
                };

                Some(token)
            }
            false => {
                dbg!(&token_buffer_as_string);
                let starts_with_apos: bool = token_buffer_as_string.starts_with('"');
                let ends_with_apos: bool = token_buffer_as_string.ends_with('"');
                if !(starts_with_apos && ends_with_apos && token_buffer_as_string.len() >= 2) {
                    return None;
                }
                let string_close_paren_index: usize = token_buffer_as_string.len() - 1;
                let token_value: REPDATA = REPDATA::STRING(
                    token_buffer_as_string[1..string_close_paren_index].to_string(),
                );
                let token: TOKEN = TOKEN {
                    kind: TOKENTYPE::STRING,
                    value: Some(token_value),
                };

                Some(token)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::match_token_buffer, primitives::TOKENTYPE};

    #[test]
    fn check_token_buffer_match() {
        let test_1: Vec<char> = "print".chars().collect::<Vec<char>>();
        let test_2: Vec<char> = "1".chars().collect::<Vec<char>>();
        let test_3: Vec<char> = r#""hello world""#.chars().collect::<Vec<char>>();

        assert_eq!(
            Some(TOKENTYPE::PRINT.into()),
            match_token_buffer(test_1, true)
        );
        // assert_eq!();
        // assert_eq!();
    }
}
