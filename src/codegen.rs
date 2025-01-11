use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
};

use crate::{
    lexer::{Lexer, Number, Operator, Token},
    parse::Parser,
};

pub struct Compiler<'ctx> {
    codegen: CodeGen<'ctx>,
    lexer: Lexer<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(src: &'ctx str, codegen: CodeGen<'ctx>, lexer: Lexer<'ctx>) -> Self {
        let lexer = Lexer::new(src);

        Self { codegen, lexer }
    }

    pub fn run(self) {
        let lexed_tokens = self.lexer.lex();
        let parser = Parser::new(lexed_tokens);
        let rpn_tokens = parser.parse_as_rpn();
        dbg!(&rpn_tokens);

        let sum = &self.codegen.compile_sum();
        let sub = &self.codegen.compile_sub();
        // let mul = &self.codegen.compile_mul();
        // let div = &self.codegen.compile_div();
        dbg!(self.codegen.module.get_functions());

        let mut stack = vec![];
        for token in rpn_tokens {
            match token {
                Token::Numeric(_) => stack.push(token),
                Token::Op(op) => {
                    // Made `mut` so they can be made into floats if the operator is division.
                    let mut y = match stack.pop() {
                        Some(Token::Numeric(Number::FloatingPoint(n))) => n,
                        _ => {
                            panic!("Ill-formed expression.")
                        }
                    };
                    let mut x = match stack.pop() {
                        Some(Token::Numeric(Number::FloatingPoint(n))) => n,
                        _ => {
                            panic!("Ill-formed expression.")
                        }
                    };
                    match op {
                        Operator::Plus => {
                            // let answer = unsafe { sum.call(x, y) };
                            // stack.push(Token::Numeric(Number::FloatingPoint(answer)));
                        }
                        Operator::Minus => {
                            // let answer = unsafe { sub.call(x, y) };
                            // stack.push(Token::Numeric(Number::FloatingPoint(answer)));
                        }
                        Operator::Asterisk => {
                            // let answer = unsafe { mul.call(x, y) };
                            // stack.push(Token::Numeric(Number::FloatingPoint(answer)));
                        }
                        Operator::Slash => {
                            // let answer = unsafe { div.call(x, y) };
                            // stack.push(Token::Numeric(Number::FloatingPoint(answer)));
                        }
                    }
                }
                _ => (),
            }
        }

        dbg!(stack);
    }
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
}

type Function = unsafe extern "C" fn(f64, f64) -> f64;

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_all(&self) -> Option<JitFunction<Function>> {
        self.compile_sum();
        self.compile_sub();
        self.compile_mul();
        self.compile_div();

        None
    }

    pub fn compile_sum(&self) -> Option<inkwell::values::FunctionValue<'_>> {
        let f64_type = self.context.f64_type();
        let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "add_entry");

        self.builder.position_at_end(basic_block);

        let operand_1 = function.get_nth_param(0).unwrap().into_float_value();
        let operand_2 = function.get_nth_param(1).unwrap().into_float_value();

        let sum = self
            .builder
            .build_float_add(operand_1, operand_2, "sum")
            .unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        unsafe { self.module.get_function("sum") }
    }

    fn compile_sub(&self) -> Option<inkwell::values::FunctionValue<'_>> {
        let f64_type = self.context.f64_type();
        let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
        let function = self.module.add_function("sub", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "sub_entry");

        self.builder.position_at_end(basic_block);

        let operand_1 = function.get_nth_param(0).unwrap().into_float_value();
        let operand_2 = function.get_nth_param(1).unwrap().into_float_value();

        let sub = self
            .builder
            .build_float_sub(operand_1, operand_2, "sub")
            .unwrap();

        self.builder.build_return(Some(&sub)).unwrap();

        unsafe { self.module.get_function("sub") }
    }

    fn compile_mul(&self) -> Option<JitFunction<Function>> {
        let f64_type = self.context.f64_type();
        let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
        let function = self.module.add_function("mul", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "mul_entry");

        self.builder.position_at_end(basic_block);

        let operand_1 = function.get_nth_param(0).unwrap().into_float_value();
        let operand_2 = function.get_nth_param(1).unwrap().into_float_value();

        let mul = self
            .builder
            .build_float_mul(operand_1, operand_2, "mul")
            .unwrap();

        self.builder.build_return(Some(&mul)).unwrap();

        unsafe { self.execution_engine.get_function("mul").ok() }
    }

    pub fn compile_div(&self) -> Option<JitFunction<Function>> {
        let f64_type = self.context.f64_type();
        let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
        let function = self.module.add_function("div", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "div_entry");

        self.builder.position_at_end(basic_block);

        let operand_1 = function.get_nth_param(0).unwrap().into_float_value();
        let operand_2 = function.get_nth_param(1).unwrap().into_float_value();

        let div = self
            .builder
            .build_float_div(operand_1, operand_2, "div")
            .unwrap();

        self.builder.build_return(Some(&div)).unwrap();

        unsafe { self.execution_engine.get_function("div").ok() }
    }
}
