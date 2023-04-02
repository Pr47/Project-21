#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    LitInt(i32),
    LitFloat(f32),

    // Keywords
    KwdInt,
    KwdFloat,
    KwdVar,
    KwdReturn,
    KwdIf,
    KwdElse,
    KwdWhile,
    KwdFor,
    KwdBreak,
    KwdContinue,

    // Operators
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpAnd,
    OpOr,
    OpNot,
    OpXor,
    OpLt,
    OpLe,
    OpGt,
    OpGe,
    OpEq,
    OpNe,

    // Symbols
    SymSemi,
    SymComma,
    SymLParen,
    SymRParen,
    SymLBrace,
    SymRBrace
}
