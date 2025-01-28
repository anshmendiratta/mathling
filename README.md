# mathling
A small math parsing language born out of frustration from having no built-in bash math expressions.

## Usage
`cargo r "<math_expr>"` where `<math_expr>` contains any valid expression using any of the supported syntax listed below.

Example: `cargo r "1 + 2/3"` => Output: `Result: 1.6666666666666665`

## Why LLVM?
This project was initially formulated to help learn LLVM by having the program parse a programming language, as opposed to just math. Since then, I've had greater motivation to parse math on the command line, but still wanted to learn some LLVM, so here we are.

Currently supported syntax:
- Addition `+`
- Subtraction `-`
- Multiplication `*`
- Division `/`

## Goals
- [x] Floating point numbers as input
- [x] Parse parentheses
- [ ] Store variables
- [ ] External links to custom functions
- [ ] Command line configuration arguments (such as how many floating point decimals to print)
