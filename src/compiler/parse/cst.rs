use smallvec::SmallVec;
use crate::io_ctx::Type21;

#[derive(Debug, Clone)]
pub struct Program {
    const_decl: Vec<ConstDecl>,
    func_decl: Vec<FuncDecl>
}

#[derive(Debug, Clone)]
pub struct ConstDecl {
    name: String,
    value: Expr
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    name: String,
    ty: SmallVec<[Type21; 2]>,
    params: SmallVec<[(Type21, String); 2]>,
    body: Option<BlockStmt>
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    stmts: Vec<Stmt>
}

