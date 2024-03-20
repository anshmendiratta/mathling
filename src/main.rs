use make_lang::parser::{make_syntax_tree, read_tokens_sequence_of_source};
use make_lang::primitives::ASTNode;
use make_lang::primitives::TOKEN;
use make_lang::traits::OptionStringify;
use make_lang::{lexer::tokenize, parser::pair_tokens};

use std::{fs::File, io::Read, io::Write};

fn main() -> std::io::Result<()> {
    let mut language_file: File = File::open("language.test")?;
    let language_file_contents: String = {
        let mut contents: String = String::new();
        language_file.read_to_string(&mut contents)?;

        contents
    };

    let tokens: Vec<TOKEN> = tokenize(language_file_contents);

    let tokens_as_vec_string: Vec<String> = tokens
        .iter()
        .map(|token| {
            dbg!(&token.as_string());
            // Removes `Some(..)` to become `..`
            token.as_string()
        })
        .collect();

    dbg!(&tokens_as_vec_string);
    let tokens_as_string = tokens_as_vec_string.join("\n");

    let mut tokens_file: File = File::create("tokens.txt")?;
    let _ = tokens_file.write_all(tokens_as_string.as_bytes());

    let tokens: Vec<TOKEN> = pair_tokens(read_tokens_sequence_of_source()?);
    dbg!(&tokens);

    let mut root_node_of_syntax_tree: ASTNode = make_syntax_tree(tokens).unwrap();
    while root_node_of_syntax_tree.next_node.is_some() {
        dbg!(&root_node_of_syntax_tree.next_node);
        root_node_of_syntax_tree = *root_node_of_syntax_tree.next_node.unwrap();
    }

    Ok(())
}
