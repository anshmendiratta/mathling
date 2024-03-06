use make_lang::lexer::tokenize;
use make_lang::primitives::TOKEN;
use make_lang::traits::Stringify;

use std::{fs::File, io::Read, io::Write};

fn main() -> std::io::Result<()> {
    let mut language_file: File = File::open("language.test")?;
    let language_file_contents: String = {
        let mut contents: String = String::new();
        language_file.read_to_string(&mut contents)?;

        contents
    };

    let tokens: Vec<TOKEN> = tokenize(language_file_contents);

    let tokens_as_vec_string: Vec<String> = tokens.as_string();
    let tokens_as_string = tokens_as_vec_string.join("\n");

    let mut tokens_file: File = File::create("tokens.txt")?;
    let _ = tokens_file.write_all(tokens_as_string.as_bytes());

    Ok(())
}
