#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use anyhow::Result;

use inkwell::context::Context;
use make_lang::{codegen::CodeGen, lexer::Lexer};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("usage: need a quoted math expression to evaluate");
    }

    let expr = args[1].clone();

    let lexer = Lexer::new(&expr);
    let tokens = lexer.lex();
    // dbg!(tokens);

    let ctx = Context::create();
    let module = ctx.create_module("sum");
    let exec_engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();
    let codegen = CodeGen {
        context: &ctx,
        module,
        builder: ctx.create_builder(),
        execution_engine: exec_engine,
    };

    let sum = codegen.compile_div().unwrap();
    let op1 = 1.;
    let op2 = 2.;

    unsafe {
        println!("{} + {} = {}", op1, op2, sum.call(op1, op2));
    };

    Ok(())
}
