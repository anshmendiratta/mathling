use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
};

use crate::{lexer::Lexer, parse::Parser};

type SumFunc = unsafe extern "C" fn(u64, u64) -> u64;
type SubFunc = unsafe extern "C" fn(u64, u64) -> u64;
type MulFunc = unsafe extern "C" fn(u64, u64) -> u64;
type DivFunc = unsafe extern "C" fn(f64, f64) -> f64;

struct Compiler<'ctx> {
    codegen: CodeGen<'ctx>,
    parser: Parser,
}

impl<'ctx> Compiler<'ctx> {
    fn new(src: &'ctx str, codegen: CodeGen<'ctx>, lexer: Lexer<'ctx>, parser: Parser) -> Self {
        let lexer = Lexer::new(src);
        let tokens = lexer.lex();
        let parser = Parser::new(tokens);

        Self { codegen, parser }
    }

    fn get_fns(&self) {}
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        let u64_type = self.context.i64_type();
        let fn_type = u64_type.fn_type(&[u64_type.into(), u64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let operand_1 = function.get_nth_param(0)?.into_int_value();
        let operand_2 = function.get_nth_param(1)?.into_int_value();

        let sum = self
            .builder
            .build_int_add(operand_1, operand_2, "sum")
            .unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        unsafe { self.execution_engine.get_function("sum").ok() }
    }

    fn compile_sub(&self) -> Option<JitFunction<SubFunc>> {
        let u64_type = self.context.i128_type();
        let fn_type = u64_type.fn_type(&[u64_type.into(), u64_type.into()], false);
        let function = self.module.add_function("sub", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let operand_1 = function.get_nth_param(0)?.into_int_value();
        let operand_2 = function.get_nth_param(1)?.into_int_value();

        let sum = self
            .builder
            .build_int_sub(operand_1, operand_2, "sub")
            .unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        unsafe { self.execution_engine.get_function("sub").ok() }
    }

    fn compile_mul(&self) -> Option<JitFunction<MulFunc>> {
        let u64_type = self.context.i128_type();
        let fn_type = u64_type.fn_type(&[u64_type.into(), u64_type.into()], false);
        let function = self.module.add_function("mul", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let operand_1 = function.get_nth_param(0)?.into_int_value();
        let operand_2 = function.get_nth_param(1)?.into_int_value();

        let sum = self
            .builder
            .build_int_mul(operand_1, operand_2, "mul")
            .unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        unsafe { self.execution_engine.get_function("mul").ok() }
    }

    pub fn compile_div(&self) -> Option<JitFunction<DivFunc>> {
        let f64_type = self.context.f64_type();
        let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
        let function = self.module.add_function("div", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let operand_1 = function.get_nth_param(0)?.into_float_value();
        let operand_2 = function.get_nth_param(1)?.into_float_value();

        let sum = self
            .builder
            .build_float_div(operand_1, operand_2, "div")
            .unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        unsafe { self.execution_engine.get_function("div").ok() }
    }
}
