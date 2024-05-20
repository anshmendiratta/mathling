use std::collections::HashSet;

use crate::types::{Error, InfixOperation, Number, RepData, Token, TokenType};

pub fn tokenize(parse_string: String) -> Vec<Token> {
    let error_msg: String;
    let mut read_buffer: Vec<&str> = Vec::new(); // NOTE: Maximal munch
    let mut tokens: Vec<Token> = Vec::new();
    let mut paren_scope: isize = 0;

    // let parse_string_lines: Vec<&str> = parse_string.split('\n');
    let parse_string_scopes = parse_string.split('(');

    for character in parse_string.chars() {
        if character == '(' {
            paren_scope += 1;
        } else if character == ')' {
            paren_scope -= 1;
        }
    }

    if paren_scope != 0 {
        error_msg = format!("Missing {} delimiters", paren_scope);
        let err = Error::ConsistentScope(error_msg);
        panic!("{:?}", err);
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

    pair_tokens(tokens)
}

pub fn pair_tokens(token_sequence: Vec<Token>) -> Vec<Token> {
    let mut paired_tokens: Vec<Token> = Vec::new();

    for (idx, token) in token_sequence.iter().enumerate() {
        match token_sequence[idx].kind {
            TokenType::PRINT | TokenType::STRING | TokenType::RETURN | TokenType::BINOP(_) => {
                if idx >= token_sequence.len() - 1 {
                    continue;
                }
                let current_token: &Token = &token_sequence[idx];
                let next_token: &Token = &token_sequence[idx + 1];
                paired_tokens.push(Token {
                    kind: current_token.kind.clone(),
                    value: next_token.value.clone(),
                });
            }
            _ => paired_tokens.push(token.clone()),
        }
    }

    paired_tokens
}

fn check_if_string_is_number(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    let mut letters_of_token: HashSet<char> = HashSet::new();
    token.chars().for_each(|character| {
        letters_of_token.insert(character);
    });

    let number_of_digits_in_token: u32 = letters_of_token
        .iter()
        .take_while(|character| character.is_ascii_digit())
        .map(|digit| match digit.to_digit(10) {
            Some(conversion) => conversion,
            None => panic!("Not a number!"),
        })
        .sum();

    number_of_digits_in_token == token.len().try_into().unwrap()
}

/// Assumes the input is not empty and valid
fn convert_string_to_digit(digit_candidate: &str) -> u32 {
    let digits_vector: Vec<u32> = digit_candidate
        .trim_start()
        .trim_end()
        .chars()
        .map(|dig_char| dig_char.to_digit(10).unwrap())
        .collect();
    let mut resulting_number: u32 = 0;

    for (place, digit) in digits_vector.iter().rev().enumerate() {
        resulting_number += digit * 10_u32.pow(place as u32);
    }

    resulting_number
}

pub fn match_token_buffer(token_buffer: Vec<char>, read_from_source: bool) -> Option<Token> {
    let token_buffer_as_string: &str = &token_buffer.iter().collect::<String>();

    // Match numbers
    if check_if_string_is_number(token_buffer_as_string) {
        let converted_number: usize = convert_string_to_digit(token_buffer_as_string) as usize;

        return Some(Token {
            kind: TokenType::NUMBER,
            value: Some(RepData::NUMBER(Number::UINT(converted_number))),
        });
    }

    // Match specific keywords
    match token_buffer_as_string {
        "print" => Some(TokenType::PRINT.into()),
        "(" => Some(TokenType::LPAREN.into()),
        ")" => Some(TokenType::RPAREN.into()),
        "+" => Some(TokenType::BINOP(InfixOperation::Addition).into()),
        "-" => Some(TokenType::BINOP(InfixOperation::Addition).into()),
        "*" => Some(TokenType::BINOP(InfixOperation::Addition).into()),
        "/" => Some(TokenType::BINOP(InfixOperation::Addition).into()),
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
        lexer::{convert_string_to_digit, match_token_buffer},
        types::{Number, RepData, Token, TokenType},
    };

    #[test]
    fn check_token_buffer_match() {
        let test_1: Vec<char> = "print".chars().collect::<Vec<char>>();
        // TODO: Setup test 2
        let test_2: Vec<char> = "1".chars().collect::<Vec<char>>();
        let test_3: Vec<char> = "'hello world'".chars().collect::<Vec<char>>();
        let test_4: Vec<char> = r#"STRING("hello world")"#.chars().collect::<Vec<char>>();

        assert_eq!(
            Some(TokenType::PRINT.into()),
            match_token_buffer(test_1, true)
        );
        assert_eq!(
            Some(Token {
                kind: TokenType::NUMBER,
                value: Some(RepData::NUMBER(Number::UINT(1))),
            }),
            match_token_buffer(test_2, true)
        );
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

    #[test]
    fn check_conversion_to_integer() {
        let input_string_1: &str = "101";
        let expected_digit_1: u32 = 101;
        let input_string_2: &str = "0";
        let expected_digit_2: u32 = 0;

        assert_eq!(expected_digit_1, convert_string_to_digit(input_string_1));
        assert_eq!(expected_digit_2, convert_string_to_digit(input_string_2));
    }
}
