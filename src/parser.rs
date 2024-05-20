use crate::{lexer::match_token_buffer, types::Token};

pub fn read_tokens_sequence_of_source() -> std::io::Result<Vec<Token>> {
    let token_sequence_from_file = std::fs::read_to_string("tokens.txt")?;
    let tokens_sequence_as_buffers: Vec<Vec<char>> = token_sequence_from_file
        .split('\n')
        .map(|c| c.to_lowercase().chars().collect())
        .collect();
    let mut tokens_sequence: Vec<Token> = Vec::new();

    for token_buffer in tokens_sequence_as_buffers {
        if let Some(matched_token) = match_token_buffer(token_buffer, false) {
            tokens_sequence.push(matched_token);
        }
    }

    Ok(tokens_sequence)
}
