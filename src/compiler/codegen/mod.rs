pub mod decl;
pub mod expr;
pub mod expr_consteval;
pub mod stmt;

use std::collections::HashMap;

use crate::compiler::codegen::decl::{CompilingFunction, FunctionInfo};
use crate::compiler::codegen::expr_consteval::ConstEvalResult;
use crate::io_ctx::Type21;
use crate::r25_300::compiled::Compiled;
use crate::value::RtValue;

#[derive(Debug)]
pub struct CodegenContext {
    compiled: Compiled,

    constant: HashMap<String, ConstEvalResult>,
    declared_func: HashMap<String, FunctionInfo>,
    compiling_func: Option<CompilingFunction>
}

#[derive(Debug, Clone, Copy)]
pub struct ExprResult {
    pub ty: Type21,
    pub value_loc: usize,
    pub consteval_value: Option<RtValue>
}

impl CodegenContext {
    pub fn new() -> Self {
        Self {
            compiled: Compiled::new(),
            constant: HashMap::new(),
            declared_func: HashMap::new(),
            compiling_func: None
        }
    }

    pub fn take(self) -> Compiled {
        self.compiled
    }
}

#[cfg(test)] mod test;
