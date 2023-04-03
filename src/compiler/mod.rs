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
    pub fn sv_error(err: SVError, line: usize) -> Self {
        Self { err: Either::Left(err), line }
    }

    pub fn syntax_error(line: usize) -> Self {
        Self { err: Either::Right(()), line }
    }
}
