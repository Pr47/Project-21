pub mod codegen;
pub mod lex;
pub mod parse;
pub mod visit;
pub mod op;

use std::fmt::Debug;
use xjbutil::either::Either;

#[derive(Debug, Clone)]
pub struct CompileError<SVError> {
    pub err: Either<SVError, ()>,
    pub line: usize
}

impl<SVError> CompileError<SVError> {
    #[cfg_attr(test, allow(unused_variables, unreachable_code))]
    pub fn sv_error(err: SVError, line: usize) -> Self {
        #[cfg(test)] panic!("unexpected semantic error, line = {}", line);
        Self { err: Either::Left(err), line }
    }

    #[cfg_attr(test, allow(unused_variables, unreachable_code))]
    pub fn syntax_error(line: usize) -> Self {
        #[cfg(test)] panic!("unexpected syntax error, line = {}", line);
        Self { err: Either::Right(()), line }
    }
}

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

impl<SVError> From<SyntaxError> for CompileError<SVError> {
    fn from(value: SyntaxError) -> Self {
        CompileError::syntax_error(value.line)
    }
}
