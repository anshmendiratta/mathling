use crate::primitives::{RepData, Token, TokenType};

pub fn tokenize(parse_string: String) -> Vec<Token> {
    let mut read_buffer: Vec<&str> = Vec::new(); // Maximal munch
    let mut tokens: Vec<Token> = Vec::new();
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

pub fn match_token_buffer(token_buffer: Vec<char>, read_from_source: bool) -> Option<Token> {
    let token_buffer_as_string: &str = &token_buffer.iter().collect::<String>();
    match token_buffer_as_string {
        "print" => Some(TokenType::PRINT.into()),
        "(" => Some(TokenType::LPAREN.into()),
        ")" => Some(TokenType::RPAREN.into()),
        _ => {
            let delimiter: char = match read_from_source {
                true => '\'',
                false => '"',
            };
            let apostrophe_matches: Vec<_> =
                token_buffer_as_string.match_indices(delimiter).collect();

            if apostrophe_matches.len() != 2 {
                return None;
            }

            let first_match_index = apostrophe_matches[0].0;
            let second_match_index = apostrophe_matches[1].0;

            let token_value: RepData = RepData::STRING(
                token_buffer_as_string[first_match_index + 1..second_match_index].to_string(),
            );
            let token: Token = Token {
                kind: TokenType::STRING,
                value: Some(token_value),
            };

            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::match_token_buffer,
        primitives::{RepData, Token, TokenType},
    };

    #[test]
    fn check_token_buffer_match() {
        let test_1: Vec<char> = "print".chars().collect::<Vec<char>>();
        // TODO: Setup test 2
        // let test_2: Vec<char> = "1".chars().collect::<Vec<char>>();
        let test_3: Vec<char> = "'hello world'".chars().collect::<Vec<char>>();
        let test_4: Vec<char> = r#"STRING("hello world")"#.chars().collect::<Vec<char>>();

        assert_eq!(
            Some(TokenType::PRINT.into()),
            match_token_buffer(test_1, true)
        );
        // NOTE: Test 2
        // assert_eq!(
        //     Some(TOKENTYPE::PRINT.into()),
        //     match_token_buffer(test_2, true)
        // );
        assert_eq!(
            Some(Token {
                kind: TokenType::STRING,
                value: Some(RepData::STRING("hello world".to_string()))
            }),
            match_token_buffer(test_3, true)
        );
        assert_eq!(
            Some(Token {
                kind: TokenType::STRING,
                value: Some(RepData::STRING("hello world".to_string()))
            }),
            match_token_buffer(test_4, false)
        );
    }
}
