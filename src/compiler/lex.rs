#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    Ident(String),
    LitInt(i32),
    LitFloat(f32),

    // Keywords
    KwdConst,
    KwdInt,
    KwdFloat,
    KwdVar,
    KwdVoid,
    KwdReturn,
    KwdIf,
    KwdElse,
    KwdWhile,
    KwdFor,
    KwdBreak,
    KwdContinue,
    KwdTrue,
    KwdFalse,

    // Operators
    OpAssign,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpAddAssign,
    OpSubAssign,
    OpMulAssign,
    OpDivAssign,
    OpModAssign,
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
    SymRBrace,
    SymLBracket,
    SymRBracket,

    // End of Input
    EOI
}

#[derive(Debug, Clone)]
pub struct Token {
    pub data: TokenData,
    pub line: usize
}

impl Token {
    pub fn new(data: TokenData, line: usize) -> Self {
        Self { data, line }
    }

    pub fn lit_int(value: i32, line: usize) -> Self {
        Self::new(TokenData::LitInt(value), line)
    }

    pub fn lit_float(value: f32, line: usize) -> Self {
        Self::new(TokenData::LitFloat(value), line)
    }

    pub fn ident(value: String, line: usize) -> Self {
        Self::new(TokenData::Ident(value), line)
    }
}
