use inkwell::context::Context;
use mathling::{
    codegen::{CodeGen, Compiler},
    lexer::Lexer,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("usage: need a quoted math expression to evaluate");
    }

    let expr = args[1].clone();
    assert!(!expr.is_empty(), "Expression cannot be empty.");

    let mut lexer = Lexer::new(&expr);
    let ctx = Context::create();
    let module = ctx.create_module("primary");
    let codegen = CodeGen {
        context: &ctx,
        module,
        builder: ctx.create_builder(),
    };

    let compiler = Compiler::new(&expr, codegen, lexer);
    let (_, output) = compiler.run().unwrap();

    println!("Result: {}", output);
}
