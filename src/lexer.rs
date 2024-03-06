use crate::primitives::TOKEN;

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

            match match_token_buffer(token_buffer.clone()) {
                Some(matched_token) => tokens.push(matched_token),
                _ => continue,
            }
        }
    }

    dbg!(paren_scope);

    tokens
}

fn match_token_buffer(token_buffer: Vec<char>) -> Option<TOKEN> {
    let token_buffer_as_string: &str = &token_buffer.iter().collect::<String>();
    let try_to_stringify: String = token_buffer_as_string.replace("\'", "");

    if token_buffer_as_string.len() >= 2 {
        if try_to_stringify.len() == &token_buffer_as_string.len() - 2 {
            return Some(TOKEN::STRING(try_to_stringify));
        }
    }

    match token_buffer_as_string {
        "print" => Some(TOKEN::PRINT),
        "(" => Some(TOKEN::LPAREN),
        ")" => Some(TOKEN::RPAREN),
        _ => None,
    }
}
