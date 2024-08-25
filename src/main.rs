use std::fs;

#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
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
}
