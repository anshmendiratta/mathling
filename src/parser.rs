


// use crate::lexer::match_buffer_token;

// /// returns a `std::io::Result<Box<ASTNode>>`
// fn make_syntax_tree(token_sequence: Vec<TOKEN>) -> std::io::Result<()> {
//     let tokens_sequence: Vec<TOKEN> = read_sequence_tokens()?;
//     let root_node: TOKEN = tokens_sequence[0];
//
//     dbg!(&root_node);
//     Ok(())
// }

// fn read_sequence_tokens() -> std::io::Result<Vec<TOKEN>> {
//     let tokens_sequence_as_strings: Vec<String> = std::fs::read_to_string("tokens.txt")?
//     .split('\n').collect();
//     let mut tokens_sequence: Vec<TOKEN> = Vec::new();
//
//     for token_string in tokens_sequence {
//         match match_token_buffer(token_string) {
//
//         }
//     }
//
//     tokens_sequence
// }
