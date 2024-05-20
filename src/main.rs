use make_lang::interpreter::eval_statement;
use make_lang::lexer::tokenize;
use make_lang::traits::OptionStringify;
use make_lang::types::Token;
use make_lang::types::TokenType;

use std::env::Args;
use std::{fs::File, io::Read, io::Write};

fn main() -> std::io::Result<()> {
    // Validate args
    let args: Args = std::env::args();
    if args.len() < 2 {
        panic!("Interpreter: Refer to a file");
    }

    // Determine file path
    let file_to_interpret_rel_path = &args.collect::<Vec<String>>()[1];
    let call_directory = std::env::current_dir()?;
    let file_to_interpret: String = format!(
        "{}/{}",
        call_directory.display(),
        file_to_interpret_rel_path
    );

    // Read file for Tokenization
    let mut language_file: File = File::open(file_to_interpret)?;
    let language_file_contents: String = {
        let mut contents: String = String::new();
        language_file.read_to_string(&mut contents)?;

        contents
    };

    // Tokenize file
    let tokens: Vec<Token> = tokenize(language_file_contents);
    dbg!(&tokens);

    // Clean up `Some(..)`
    let tokens_as_vec_string: Vec<String> = tokens
        .iter()
        .map(|token| {
            if token.kind == TokenType::STRING {
                token.value.option_as_string()
            } else {
                token.as_string()
            }
        })
        .collect();

    let tokens_as_string = tokens_as_vec_string.join("\n");

    // Write tokens
    let mut tokens_file: File = File::create("tokens.txt")?;
    tokens_file.write_all(tokens_as_string.as_bytes())?;

    // Evaluate tokens
    for statement in tokens {
        eval_statement(&statement);
    }

    Ok(())
}
