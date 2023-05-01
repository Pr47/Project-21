use smallvec::SmallVec;
use crate::compiler::op::{BinaryOp, UnaryOp};
use crate::io_ctx::Type21;

#[derive(Default, Debug, Clone)]
pub struct Program {
    pub const_decl: Vec<ConstDecl>,
    pub func_decl: Vec<FuncDecl>
}

#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub name: String,
    pub value: Expr
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub name: String,
    pub ty: SmallVec<[Type21; 2]>,
    pub params: SmallVec<[(Type21, String); 2]>,
    pub body: Option<Box<BlockStmt>>
}

#[derive(Debug, Clone)]
pub enum Stmt {
    DeclStmt(Box<VarDecl>),
    ExprStmt(Expr),
    IfStmt(Box<IfStmt>),
    BlockStmt(Box<BlockStmt>),
    WhileStmt(Box<WhileStmt>),
    ForStmt(Box<ForStmt>),
    ReturnStmt(Option<Expr>),
    BreakStmt(usize),
    ContinueStmt(usize),
    YieldStmt(usize)
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub stmts: SmallVec<[Stmt; 4]>
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub ty: Option<Type21>,
    pub name: String,
    pub init: Option<Expr>
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub then: Stmt,
    pub else_: Option<Stmt>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct ForStmt {
    pub init: Option<Expr>,
    pub cond: Option<Expr>,
    pub step: Option<Expr>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub enum Expr {
    AtomicExpr(Box<AtomicExpr>),
    AssignExpr(Box<AssignExpr>),
    MultiAssignExpr(Box<MultiAssignExpr>),
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    FuncCall(Box<FuncCall>)
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct MultiAssignExpr {
    pub names: SmallVec<[String; 2]>,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum AtomicExpr {
    Ident(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Paren(Expr),
    TypeCast(TypeCast),
    FuncCall(FuncCall)
}

#[derive(Debug, Clone)]
pub struct TypeCast {
    pub dest: Type21,
    pub expr: Expr
}

#[derive(Debug, Clone)]
pub struct FuncCall {
    pub name: String,
    pub args: SmallVec<[Expr; 4]>,
}
