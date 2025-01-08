#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::fs;

use anyhow::Result;
use nom::character::complete::char;
use nom::{bytes::complete::is_not, sequence::delimited};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("usage: reference a file to run");
    }

    // Get file's "raw name"
    let file = args[1].as_str();
    let path_index = file.rfind("/").map(|idx| idx + 1).unwrap_or(0);
    let extension_index = file.rfind(".").unwrap_or(file.len());
    let raw_filename = &file[path_index..extension_index];

    let source_code =
        fs::read_to_string(file).unwrap_or_else(|_| panic!("could not read source of: {}", file));
    let source_code_lines = source_code.split('\n').collect::<Vec<_>>();

    let mut scope: usize = 0;

    let (removed, stripped) = delimit_parens(source_code_lines[0]).unwrap_or_default();
    if removed.len() > 0 {
        scope += 1;
    }

    dbg!(stripped);
    dbg!(scope);

    Ok(())
}

fn delimit_parens(tokens: &str) -> Result<(&str, &str), nom::Err<()>> {
    delimited(char('('), is_not(")"), char(')'))(tokens)
}
