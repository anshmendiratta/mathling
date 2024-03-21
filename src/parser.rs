use crate::{
    lexer::match_token_buffer,
    primitives::{ASTNode, TOKEN, TOKENTYPE},
};

/// returns a `std::io::Result<Box<ASTNode>>`
pub fn make_syntax_tree(token_sequence: Vec<TOKEN>) -> std::io::Result<ASTNode> {
    let root_node: ASTNode = ASTNode {
        left_child: token_sequence[0].clone(),
        next_node: None,
    };

    let number_of_tokens: usize = token_sequence.len();
    (0..=number_of_tokens - 1).for_each(|token_idx| {
        if token_idx < number_of_tokens - 1 {
            let mut most_recent_next_node: Option<Box<ASTNode>> = root_node.next_node.clone();
            while most_recent_next_node.is_some() {
                most_recent_next_node = most_recent_next_node.clone().unwrap().next_node;
            }

            most_recent_next_node.unwrap().next_node = Some(Box::new(ASTNode {
                left_child: token_sequence[token_idx].clone(),
                next_node: None,
            }))
        }
    });

    Ok(root_node)
}

pub fn read_tokens_sequence_of_source() -> std::io::Result<Vec<TOKEN>> {
    let token_sequence_from_file = std::fs::read_to_string("tokens.txt")?;
    let tokens_sequence_as_buffers: Vec<Vec<char>> = token_sequence_from_file
        .split('\n')
        .map(|c| c.to_lowercase().chars().collect())
        .collect();
    // dbg!(&tokens_sequence_as_buffers);
    let mut tokens_sequence: Vec<TOKEN> = Vec::new();

    for token_buffer in tokens_sequence_as_buffers {
        if let Some(matched_token) = match_token_buffer(token_buffer, false) {
            dbg!(&matched_token);
            tokens_sequence.push(matched_token);
        }
    }

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
