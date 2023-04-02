use xjbutil::either::Either;
use crate::compiler::op::{BinaryOp, UnaryOp};
use crate::io_ctx::Type21;

pub trait SyntaxVisitor {
    type ExprResult;
    type StmtResult;
    type DeclResult;

    fn visit_lit_int(&mut self, value: i32) -> Self::ExprResult;
    fn visit_lit_float(&mut self, value: f32) -> Self::ExprResult;
    fn visit_lit_bool(&mut self, value: bool) -> Self::ExprResult;
    fn visit_uop(&mut self, op: UnaryOp, operand: Self::ExprResult) -> Self::ExprResult;
    fn visit_bin_op(
        &mut self,
        op: BinaryOp,
        lhs: Self::ExprResult,
        rhs: Self::ExprResult
    ) -> Self::ExprResult;
    fn visit_assign(
        &mut self,
        names: Vec<&str>,
        value: Self::ExprResult
    ) -> Self::ExprResult;
    fn visit_type_cast(
        &mut self,
        ty: Type21,
        operand: Self::ExprResult
    ) -> Self::ExprResult;
    fn visit_call(
        &mut self,
        name: &str,
        args: Vec<Self::ExprResult>
    ) -> Self::ExprResult;

    fn visit_expr_stmt(&mut self, expr: Self::ExprResult) -> Self::StmtResult;
    fn visit_decl_stmt(&mut self, decl: Self::DeclResult) -> Self::StmtResult;
    fn visit_if_stmt(
        &mut self,
        cond: Self::ExprResult,
        then: Self::StmtResult,
        otherwise: Option<Self::StmtResult>
    ) -> Self::StmtResult;
    fn visit_while_stmt(
        &mut self,
        cond: Self::ExprResult,
        body: Self::StmtResult
    ) -> Self::StmtResult;
    fn visit_for_stmt(
        &mut self,
        init: Either<Self::ExprResult, Self::DeclResult>,
        cond: Self::ExprResult,
        step: Self::ExprResult,
        body: Self::StmtResult
    ) -> Self::StmtResult;

    fn visit_var_decl(
        &mut self,
        ty: Option<Type21>,
        name: &str,
        init: Option<Self::ExprResult>
    ) -> Self::DeclResult;
    fn visit_func_decl(
        &mut self,
        ty: Option<Type21>,
        name: &str,
        params: Vec<(Option<Type21>, &str)>,
        body: Vec<Self::ExprResult>
    ) -> Self::DeclResult;
}
