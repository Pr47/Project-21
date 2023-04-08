use crate::compiler::lex::TokenData;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not
}

impl From<&TokenData> for UnaryOp {
    fn from(value: &TokenData) -> Self {
        match value {
            TokenData::OpSub => UnaryOp::Negate,
            TokenData::OpNot => UnaryOp::Not,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl TryFrom<&TokenData> for BinaryOp {
    type Error = ();

    fn try_from(value: &TokenData) -> Result<Self, ()> {
        Ok(match value {
            TokenData::OpAdd => BinaryOp::Add,
            TokenData::OpSub => BinaryOp::Sub,
            TokenData::OpMul => BinaryOp::Mul,
            TokenData::OpDiv => BinaryOp::Div,
            TokenData::OpMod => BinaryOp::Mod,
            TokenData::OpEq => BinaryOp::Eq,
            TokenData::OpNe => BinaryOp::Ne,
            TokenData::OpLt => BinaryOp::Lt,
            TokenData::OpLe => BinaryOp::Le,
            TokenData::OpGt => BinaryOp::Gt,
            TokenData::OpGe => BinaryOp::Ge,
            TokenData::OpAnd => BinaryOp::And,
            TokenData::OpOr => BinaryOp::Or,
            _ => {
                return Err(())
            }
        })
    }
}

impl BinaryOp {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Add | BinaryOp::Sub => 1,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 2,
            BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 3,
            BinaryOp::And => 4,
            BinaryOp::Or => 5
        }
    }
}
