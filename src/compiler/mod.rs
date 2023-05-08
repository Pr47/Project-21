pub mod lex;
pub mod parse;
pub mod op;
pub mod codegen;

use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub line: usize
}

impl SyntaxError {
    #[cfg_attr(test, allow(unused_variables, unreachable_code))]
    pub fn new(line: usize) -> Self {
        #[cfg(test)] panic!("unexpected syntax error, line = {}", line);
        Self { line }
    }
}
