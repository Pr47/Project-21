pub mod consteval;

use std::collections::HashMap;
use crate::io_ctx::Type21;
use crate::r25_300::compiled::Compiled;
use crate::value::RtValue;

#[derive(Debug)]
pub struct CodegenContext {
    compiled: Compiled,

    constant: HashMap<String, ConstEvalResult>
}

#[derive(Debug, Clone, Copy)]
pub struct ConstEvalResult {
    pub ty: Type21,
    pub value: RtValue
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
            constant: HashMap::new()
        }
    }

    pub fn take(self) -> Compiled {
        self.compiled
    }
}

#[cfg(test)] mod test;
