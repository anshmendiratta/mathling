# make_lang / `inkwell`
An foray into LLVM manifested as a language.

## What?
[todo]

## How?
Using LLVM@18.

## Why?
Language development has always greatly interested me, so it was about time I tried to learn more. This is not meant to be a functional/ready-to-use-for-anyone language. Further, this is only being tested on a 2020 MacBook Pro using an M1 chip, so if this does not work for you, good luck.

## Git Branches
When beginning this project, the structure of the program and interpreter did not appreciate [Abstract Syntax Trees](https://en.wikipedia.org/wiki/Abstract_syntax_tree). Because of this, the project was branched into two versions: one which removed ASTs, and one which retained it. Now, there is a third branch — this one — that does appreciate previous work.

1. `no-ast`: No ASTs.
2. `master`: With ASTs.
3. `inkwell`: Using documented research and external crates for language development.

## Goals
- [ ] Add parsing of numbers
- [ ] Implement evaluation of expressions
    - [ ] For numbers
    - [ ] Variables
- [ ] Store variables
