use crate::error::ParseError;
use crate::parse::Parser;
use crate::{Span, Token, TokenType};
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::{builder::Builder, context::Context, module::Module};

use crate::IResult;
use crate::lexer::{BinOp, Lexer};

pub struct Compiler<'ctx> {
    codegen: CodeGen<'ctx>,
    lexer: Lexer<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(src: &'ctx str, codegen: CodeGen<'ctx>, lexer: Lexer<'ctx>) -> Self {
        let lexer = Lexer::new(src);
        Self { codegen, lexer }
    }

    pub fn run(mut self) -> IResult<'ctx, f32> {
        let (_, lexed_tokens) = self.lexer.lex()?;
        let parser = Parser::new(lexed_tokens);
        let (_, (rpn_tokens, var_symbol_table)) = parser.parse()?;
        let fn_symbol_table = self.codegen.compile_all_fns();
        let execution_engine: ExecutionEngine<'ctx> = self
            .codegen
            .module
            .create_jit_execution_engine(inkwell::OptimizationLevel::Aggressive)
            .unwrap();

        type Function = unsafe extern "C" fn(f32, f32) -> f32;
        // Need to get all functions here because doing so compiles the module. For some reason, you can not recompile the module after the first time.
        let sum = { unsafe { execution_engine.get_function("sum").ok().unwrap() } }
            as JitFunction<'ctx, Function>;
        let sub = { unsafe { execution_engine.get_function("sub").ok().unwrap() } }
            as JitFunction<'ctx, Function>;
        let mul = { unsafe { execution_engine.get_function("mul").ok().unwrap() } }
            as JitFunction<'ctx, Function>;
        let div = { unsafe { execution_engine.get_function("div").ok().unwrap() } }
            as JitFunction<'ctx, Function>;

        let mut stack: Vec<Token> = vec![];
        for token in rpn_tokens {
            match token {
                Token {
                    token_type: TokenType::Fp(ref n),
                    ..
                } => stack.push(token),
                Token {
                    token_type: TokenType::BinOp(op),
                    ..
                } => {
                    // Made `mut` so they can be made into floats if the operator is division.
                    let mut y = match stack.pop() {
                        Some(Token {
                            token_type: TokenType::Fp(n),
                            ..
                        }) => n,
                        Some(Token {
                            token_type: TokenType::Id(id),
                            ..
                        }) => *var_symbol_table
                            .lookup(&id)
                            .expect("Found identifier not used in assignments."),
                        _ => {
                            panic!("Ill-formed expression.")
                        }
                    };
                    let mut x = match stack.pop() {
                        Some(Token {
                            token_type: TokenType::Fp(n),
                            ..
                        }) => n,
                        Some(Token {
                            token_type: TokenType::Id(id),
                            ..
                        }) => *var_symbol_table
                            .lookup(&id)
                            .expect("Found identifier not used in assignments."),
                        _ => {
                            panic!("Ill-formed expression.")
                        }
                    };
                    match op {
                        BinOp::Plus => {
                            let result = unsafe { sum.call(x, y) };
                            stack.push(Token {
                                token_type: TokenType::Fp(result),
                                location_col: None,
                            });
                        }
                        BinOp::Minus => {
                            let result = unsafe { sub.call(x, y) };
                            stack.push(Token {
                                token_type: TokenType::Fp(result),
                                location_col: None,
                            });
                        }
                        BinOp::Times => {
                            let result = unsafe { mul.call(x, y) };
                            stack.push(Token {
                                token_type: TokenType::Fp(result),
                                location_col: None,
                            });
                        }
                        BinOp::Divide => {
                            let result = unsafe { div.call(x, y) };
                            stack.push(Token {
                                token_type: TokenType::Fp(result),
                                location_col: None,
                            });
                        }
                        BinOp::Equal => unreachable!(),
                    }
                }
                _ => (),
            }
        }

        assert!(
            stack.len() == 1,
            "Evaluator stack is not of length 1. Either all variables introduced were not used, or there exists an ill-formed expression."
        );
        match stack.first().unwrap() {
            Token {
                token_type: TokenType::Fp(n),
                ..
            } => Ok::<_, ParseError>((Span::new(""), n)),
            _ => panic!("Error: After eval, last token is NOT a number."),
        };

        Err(nom::Err::Error(ParseError::new(
            Span::new(""),
            "I shouldn't run...".into(),
        )))
    }
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl CodeGen<'_> {
    pub fn compile_all_fns(&self) {
        self.compile_sum();
        self.compile_sub();
        self.compile_mul();
        self.compile_div();
    }

    pub fn compile_sum(&self) {
        let f32_type = self.context.f32_type();
        let fn_type = f32_type.fn_type(&[f32_type.into(), f32_type.into()], false);
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
    }

    fn compile_sub(&self) {
        let f32_type = self.context.f32_type();
        let fn_type = f32_type.fn_type(&[f32_type.into(), f32_type.into()], false);
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
    }

    fn compile_mul(&self) {
        let f32_type = self.context.f32_type();
        let fn_type = f32_type.fn_type(&[f32_type.into(), f32_type.into()], false);
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
    }

    pub fn compile_div(&self) {
        let f32_type = self.context.f32_type();
        let fn_type = f32_type.fn_type(&[f32_type.into(), f32_type.into()], false);
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
    }
}
