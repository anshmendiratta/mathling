# lispling
A small math parsing language born out of frustration from having no built-in bash math expressions.

## Why LLVM?
This project was initially formulated to help learn LLVM by having the program parse a programming language, as opposed to just math. Since then, I've had greater motivation to parse math on the command line, but still wanted to learn LLVM, so here we are.

Currently supported syntax:
- todo!()

## How?
Parsing using `nom` and `inkwell`. Using LLVM@18.

## Why?
Language development has always greatly interested me, so it was about time I tried to learn more. This is not meant to be a functional/ready-to-use-for-anyone language. Further, this is only being tested on Linux Mint, so if this does not work for you, good luck.

## Goals
- [ ] Add parsing of numbers
- [ ] Implement evaluation of expressions
    - [ ] For numbers
    - [ ] Variables
- [ ] Store variables
