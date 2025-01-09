#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use anyhow::Result;

use make_lang::Lexer;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("usage: need a quoted math expression to evaluate");
    }

    let expr = args[1].clone();

    let mut lexer = Lexer::new(&expr);
    let tokens = lexer.lex();
    dbg!(tokens);

    Ok(())
}
