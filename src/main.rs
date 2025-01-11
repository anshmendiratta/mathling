use anyhow::Result;

use inkwell::context::Context;
use lispling::{
    codegen::{CodeGen, Compiler},
    lexer::Lexer,
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("usage: need a quoted math expression to evaluate");
    }

    let expr = args[1].clone();

    let ctx = Context::create();
    let module = ctx.create_module("primary");
    let codegen = CodeGen {
        context: &ctx,
        module,
        builder: ctx.create_builder(),
    };
    let lexer = Lexer::new(&expr);
    let compiler = Compiler::new(&expr, codegen, lexer);
    let output = compiler.run();

    println!("Result: {}", output);

    Ok(())
}
