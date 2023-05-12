use crate::compiler::codegen::CodegenContext;
use crate::compiler::parse::cst::Expr;
use crate::io_ctx::Type21;
use crate::value::RtValue;

pub struct ExprResult {
    ty: Type21,
    loc: Option<usize>,
    consteval_value: Option<RtValue>
}

impl CodegenContext {
    pub fn codegen_expr(&mut self, expr: &Expr) -> Result<ExprResult, String> {
        todo!()
    }
}
