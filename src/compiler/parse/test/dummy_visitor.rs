use std::convert::Infallible;
use crate::compiler::lex::TokenData;
use crate::compiler::op::{BinaryOp, UnaryOp};
use crate::compiler::visit::SyntaxVisitor;
use crate::io_ctx::Type21;

pub struct DummyVisitor();

impl SyntaxVisitor for DummyVisitor {
    type ExprResult = ();
    type StmtResult = ();
    type DeclResult = ();
    type Error = Infallible;

    fn visit_ident(&mut self, _ident: &str) -> Self::ExprResult {}

    fn visit_lit_int(&mut self, _value: i32) -> Self::ExprResult {}

    fn visit_lit_float(&mut self, _value: f32) -> Self::ExprResult {}

    fn visit_lit_bool(&mut self, _value: bool) -> Self::ExprResult {}

    fn visit_uop(
        &mut self,
        _op: UnaryOp,
        _operand: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        Ok(())
    }

    fn visit_bin_op(
        &mut self,
        _op: BinaryOp,
        _lhs: Self::ExprResult,
        _rhs: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        Ok(())
    }

    fn visit_assign(
        &mut self,
        _assign_op: &TokenData,
        _names: &str,
        _value: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        Ok(())
    }

    fn visit_assign2(
        &mut self,
        _names: &[&str],
        _value: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        Ok(())
    }

    fn visit_type_cast(
        &mut self,
        _ty: Type21,
        _operand: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        Ok(())
    }

    fn visit_call(
        &mut self,
        _name: &str,
        _args: &[Self::ExprResult]
    ) -> Result<Self::ExprResult, Self::Error> {
        Ok(())
    }

    fn visit_expr_stmt(&mut self, _expr: Self::ExprResult) -> Self::StmtResult {}

    fn visit_decl_stmt(
        &mut self,
        _decl: Self::DeclResult
    ) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        _cond: Self::ExprResult,
        _then: Self::StmtResult,
        _otherwise: Option<Self::StmtResult>
    ) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_while_stmt(
        &mut self,
        _cond: Self::ExprResult,
        _body: Self::StmtResult
    ) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_for_stmt(
        &mut self,
        _init: Option<Self::ExprResult>,
        _cond: Option<Self::ExprResult>,
        _step: Option<Self::ExprResult>,
        _body: Self::StmtResult
    ) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_break_stmt(&mut self) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_continue_stmt(&mut self) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_return_stmt(
        &mut self,
        _value: Option<Self::ExprResult>
    ) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_block_stmt(
        &mut self,
        _stmts: &[Self::StmtResult]
    ) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_var_decl(
        &mut self,
        _ty: Option<Type21>,
        _name: &str,
        _init: Option<Self::ExprResult>
    ) -> Result<Self::DeclResult, Self::Error> {
        Ok(())
    }

    fn visit_func_decl(
        &mut self,
        _ty: &[Type21],
        _name: &str,
        _params: &[(Type21, &str)],
        _body: Option<Self::StmtResult>
    ) -> Result<Self::DeclResult, Self::Error> {
        Ok(())
    }

    fn visit_const_decl(
        &mut self,
        _name: &str,
        _init: Self::ExprResult
    ) -> Result<Self::DeclResult, Self::Error> {
        Ok(())
    }
}