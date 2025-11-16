use mathling::lexer::Lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("usage: need a quoted math expression to evaluate");
    }

    let expr = args[1].clone();
    assert!(!expr.is_empty(), "Expression cannot be empty.");

    // let ctx = Context::create();
    // let module = ctx.create_module("primary");
    // let codegen = CodeGen {
    //     context: &ctx,
    //     module,
    //     builder: ctx.create_builder(),
    // };
    let mut lexer = Lexer::new(&expr);
    let (_, statements) = lexer.lex().unwrap();
    for statement in statements {
        dbg!(statement);
    }

    // let compiler = Compiler::new(&expr, codegen, lexer);
    // let output = compiler.run()?;

    // println!("Result: {}", output);
}
