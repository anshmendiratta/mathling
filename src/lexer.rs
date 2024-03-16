use crate::primitives::{REPDATA, TOKEN, TOKENTYPE};

pub fn tokenize(parse_string: String) -> Vec<TOKEN> {
    let mut read_buffer: Vec<&str> = Vec::new(); // Maximal munch
    let mut tokens: Vec<TOKEN> = Vec::new();
    let paren_scope: usize = 0;

    let parse_string_lines = parse_string.split('(');

    for character in parse_string.chars() {
        if character == '(' {
            let _ = paren_scope.checked_add(1);
        } else if character == ')' {
            let _ = paren_scope.checked_sub(1);
        }
    }

    for line in parse_string_lines {
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
                let try_to_stringify: String = token_buffer_as_string.replace('\'', "");
                if try_to_stringify.len() < 2
                    || try_to_stringify.len() != token_buffer_as_string.len() - 2
                {
                    return None;
                }

                let token_value: REPDATA = REPDATA::STRING(try_to_stringify);
                let token: TOKEN = TOKEN {
                    kind: TOKENTYPE::STRING,
                    value: Some(token_value),
                };

                Some(token)
            }
            false => {
                let try_to_stringify: String = token_buffer_as_string.replace('\"', "");
                if try_to_stringify.len() <= 5
                    || try_to_stringify.len() != token_buffer_as_string.len() - 2
                {
                    return None;
                }

                let string_length: usize = try_to_stringify.len();
                let just_string: String = token_buffer_as_string[8..=string_length - 1].to_string();
                let token_value: REPDATA = REPDATA::STRING(just_string);
                let token: TOKEN = TOKEN {
                    kind: TOKENTYPE::STRING,
                    value: Some(token_value),
                };

                Some(token)
            }
        },
    }
}
