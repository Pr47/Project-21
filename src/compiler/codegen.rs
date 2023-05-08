use std::collections::HashMap;
use crate::compiler::parse::cst::*;
use crate::io_ctx::Type21;
use crate::r25_300::compiled::Compiled;
use crate::value::RtValue;

pub struct CodegenContext {
    compiled: Compiled,

    constant: HashMap<String, RtValue>
}

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

    pub fn visit_const_decl(&mut self, const_decl: &ConstDecl) -> Result<(), String> {
        todo!()
    }

    pub fn visit_expr(&mut self, expr: &Expr) -> Result<ExprResult, String> {
        match expr {
            Expr::AtomicExpr(a) => visit_atomic_expr(a),
            Expr::AssignExpr(_) => todo!(),
            Expr::MultiAssignExpr(_) => todo!(),
            Expr::BinaryExpr(_) => todo!(),
            Expr::UnaryExpr(_) => todo!(),
            Expr::FuncCall(_) => todo!()
        }
    }

    pub fn visit_atomic_expr(&mut self, expr: &AtomicExpr) -> Result<ExprResult, String> {
        match expr {
            AtomicExpr::Ident(ident) => {
                todo!()
            }
            AtomicExpr::Integer(i) => {
                Ok(ExprResult {
                    ty: Type21::Int,
                    value_loc: self.compiled.code.len(),
                    consteval_value: Some(RtValue::Int(*i))
                })
            }
            AtomicExpr::Float(f) => {
                Ok(ExprResult {
                    ty: Type21::Float,
                    value_loc: self.compiled.code.len(),
                    consteval_value: Some(RtValue::Float(*f))
                })
            }
            AtomicExpr::Bool(b) => {
                Ok(ExprResult {
                    ty: Type21::Bool,
                    value_loc: self.compiled.code.len(),
                    consteval_value: Some(RtValue::Bool(*b))
                })
            }
            AtomicExpr::Paren(inner) => self.visit_expr(inner),
            AtomicExpr::TypeCast(_) => todo!(),
            AtomicExpr::FuncCall(_) => todo!()
        }
    }
}