use crate::{
    lexer::match_token_buffer,
    primitives::{ASTNode, TOKEN, TOKENTYPE},
};

/// returns a `std::io::Result<Box<ASTNode>>`
fn make_syntax_tree(token_sequence: Vec<TOKEN>) -> std::io::Result<()> {
    let tokens_sequence: Vec<TOKEN> = pair_tokens(read_tokens_sequence()?);
    dbg!(&token_sequence);

    let root_node: TOKEN = tokens_sequence[0].clone();
    let _all_nodes: Vec<ASTNode> = Vec::new();

    dbg!(&root_node);
    Ok(())
}

pub fn read_tokens_sequence() -> std::io::Result<Vec<TOKEN>> {
    let token_sequence_from_file = std::fs::read_to_string("tokens.txt")?;
    let tokens_sequence_as_buffers: Vec<Vec<char>> = token_sequence_from_file
        .split('\n')
        .map(|c| c.to_lowercase().chars().collect())
        .collect();
    let mut tokens_sequence: Vec<TOKEN> = Vec::new();

    for token_buffer in tokens_sequence_as_buffers {
        if let Some(matched_token) = match_token_buffer(token_buffer, false) {
            tokens_sequence.push(matched_token);
        }
    }

    dbg!(&tokens_sequence);
    Ok(tokens_sequence)
}

pub fn pair_tokens(token_sequence: Vec<TOKEN>) -> Vec<TOKEN> {
    let mut paired_tokens: Vec<TOKEN> = Vec::new();

    for (idx, token) in token_sequence.iter().enumerate() {
        match token_sequence[idx].kind {
            TOKENTYPE::PRINT | TOKENTYPE::STRING | TOKENTYPE::RETURN => {
                if idx >= token_sequence.len() - 1 {
                    continue;
                }
                let next_token = token_sequence[idx + 1].clone();
                paired_tokens.push(TOKEN {
                    kind: token_sequence[idx].clone().kind,
                    value: next_token.value,
                });
            }
            _ => paired_tokens.push(token.clone()),
        }
    }

    paired_tokens
}
