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
pub enum Stmt {
    LocalDeclStmt(LocalDecl),
    ExprStmt(Expr),
    IfStmt(IfStmt),
    BlockStmt(BlockStmt),
    WhileStmt(WhileStmt),
    ForStmt(ForStmt),
    ReturnStmt(Option<Expr>),
    BreakStmt(usize),
    ContinueStmt(usize),
    YieldStmt(usize)
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    stmts: Vec<Stmt>
}

#[derive(Debug, Clone)]
pub struct LocalDecl {
    ty: Type21,
    name: String,
    value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    cond: Expr,
    then: Box<Stmt>,
    else_: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    cond: Expr,
    body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ForStmt {
    init: Option<Expr>,
    cond: Option<Expr>,
    step: Option<Expr>,
    body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    AssignExpr(Box<AssignExpr>),
    MultiAssignExpr(Box<MultiAssignExpr>),
    BinaryExpr(Box<BinaryExpr>),
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    name: String,
    value: Expr,
}

#[derive(Debug, Clone)]
pub struct MultiAssignExpr {
    names: Vec<String>,
    value: Expr,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    op: BinaryOp,
    lhs: Expr,
    rhs: Expr,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    And,
    Or,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    op: UnaryOp,
    expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum AtomicExpr {
    Ident(String),
    Number(f64),
    String(String),
    Paren(Expr),
    FuncCall(FuncCall),
}

#[derive(Debug, Clone)]
pub struct FuncCall {
    name: String,
    args: Vec<Expr>,
}
