use make_lang::parser::read_tokens_sequence;
use make_lang::primitives::TOKEN;
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
            // Removes `Some(..)` to become `..`
            if token.as_string().replace("Some(", "") != token.as_string() {
                let token_string_length = token.as_string().len();
                return token.as_string()[5..=token_string_length - 2].to_string();
            }

            token.as_string()
            // format!("{:?}", token)
        })
        .collect();

    let tokens_as_string = tokens_as_vec_string.join("\n");

    let mut tokens_file: File = File::create("tokens.txt")?;
    let _ = tokens_file.write_all(tokens_as_string.as_bytes());

    let tokens: Vec<TOKEN> = pair_tokens(read_tokens_sequence()?);
    dbg!(tokens);

    Ok(())
}
