use std::collections::HashMap;
use crate::compiler::codegen::expr_consteval::ConstEvalResult;
use crate::compiler::codegen_c::decl::FunctionInfo;

pub mod decl;

#[derive(Debug)]
pub struct CCodegenContext {
    code: String,
    indent: u32,

    constant: HashMap<String, ConstEvalResult>,
    declared_func: HashMap<String, FunctionInfo>,
}

impl CCodegenContext {
    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn dedent(&mut self) {
        self.indent -= 1;
    }

    pub fn insert_code(&mut self, code: &str) {
        for _ in 0..self.indent {
            self.code.push_str("    ");
        }

        self.code.push_str(code);
        if !code.ends_with('\n') {
            self.code.push('\n');
        }
    }
}
