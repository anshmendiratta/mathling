use crate::{
    lexer::match_token_buffer,
    primitives::{Token, TokenType},
};

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

pub fn pair_tokens(token_sequence: Vec<Token>) -> Vec<Token> {
    let mut paired_tokens: Vec<Token> = Vec::new();

    for (idx, token) in token_sequence.iter().enumerate() {
        match token_sequence[idx].kind {
            TokenType::PRINT | TokenType::STRING | TokenType::RETURN => {
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
