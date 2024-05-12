use make_lang::interpreter::eval_statement;
use make_lang::parser::read_tokens_sequence_of_source;
use make_lang::primitives::Token;
use make_lang::primitives::TokenType;
use make_lang::traits::OptionStringify;
use make_lang::{lexer::tokenize, parser::pair_tokens};

use std::{fs::File, io::Read, io::Write};

fn main() -> std::io::Result<()> {
    // Tokenize and write to file
    {
        let mut language_file: File = File::open("language.test")?;
        let language_file_contents: String = {
            let mut contents: String = String::new();
            language_file.read_to_string(&mut contents)?;

            contents
        };

        let tokens: Vec<Token> = tokenize(language_file_contents);

        let tokens_as_vec_string: Vec<String> = tokens
            .iter()
            .map(|token| {
                // Removes `Some(..)` to become `..`
                if token.kind == TokenType::STRING {
                    token.value.option_as_string()
                } else {
                    token.as_string()
                }
            })
            .collect();

        let tokens_as_string = tokens_as_vec_string.join("\n");

        let mut tokens_file: File = File::create("tokens.txt")?;
        let _ = tokens_file.write_all(tokens_as_string.as_bytes());
    }

    let tokens: Vec<Token> = pair_tokens(read_tokens_sequence_of_source()?);
    tokens
        .iter()
        .for_each(|statement| eval_statement(statement));

    Ok(())
}
