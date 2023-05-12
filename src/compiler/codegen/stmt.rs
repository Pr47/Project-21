use crate::compiler::codegen::CodegenContext;
use crate::compiler::parse::cst::*;

impl CodegenContext {
    pub fn codegen_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::DeclStmt(var_decl) => self.visit_var_decl(var_decl),
            Stmt::ExprStmt(expr, line) => self.codegen_expr(expr).map_err(|e| format!("行 {}: {}", line, e)),
            Stmt::IfStmt(if_stmt) => self.codegen_if_stmt(if_stmt),
            Stmt::BlockStmt(block_stmt) => self.codegen_block_stmt(block_stmt),
            Stmt::WhileStmt(while_stmt) => self.codegen_while_stmt(while_stmt),
            Stmt::ForStmt(for_stmt) => self.codegen_for_stmt(for_stmt),
            Stmt::ReturnStmt(return_stmt) => self.codegen_return_stmt(return_stmt),
            Stmt::MultiReturnStmt(return_stmt) => self.codegen_multi_return_stmt(return_stmt),
            Stmt::BreakStmt(break_stmt) => self.codegen_break_stmt(break_stmt),
            Stmt::ContinueStmt(continue_stmt) => self.codegen_continue_stmt(continue_stmt),
            Stmt::YieldStmt(yield_stmt) => self.codegen_yield_stmt(yield_stmt)
        }
    }
}
