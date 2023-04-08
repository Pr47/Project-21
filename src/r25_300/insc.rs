use crate::value::RtValue;
#[cfg(test)] use variant_count::VariantCount;

#[cfg_attr(test, derive(VariantCount))]
#[derive(Debug, Clone)]
pub enum Insc {
    Const { value: RtValue, dst: usize },

    AddInt { lhs: usize, rhs: usize, dst: usize },
    AddFloat { lhs: usize, rhs: usize, dst: usize },
    SubInt { lhs: usize, rhs: usize, dst: usize },
    SubFloat { lhs: usize, rhs: usize, dst: usize },
    MulInt { lhs: usize, rhs: usize, dst: usize },
    MulFloat { lhs: usize, rhs: usize, dst: usize },
    DivInt { lhs: usize, rhs: usize, dst: usize },
    DivFloat { lhs: usize, rhs: usize, dst: usize },
    ModInt { lhs: usize, rhs: usize, dst: usize },

    NegateInt { src: usize, dst: usize },
    NegateFloat { src: usize, dst: usize },

    Eq { lhs: usize, rhs: usize, dst: usize },
    Ne { lhs: usize, rhs: usize, dst: usize },

    LtInt { lhs: usize, rhs: usize, dst: usize },
    LtFloat { lhs: usize, rhs: usize, dst: usize },
    LeInt { lhs: usize, rhs: usize, dst: usize },
    LeFloat { lhs: usize, rhs: usize, dst: usize },

    And { lhs: usize, rhs: usize, dst: usize },
    Or { lhs: usize, rhs: usize, dst: usize },
    Not { src: usize, dst: usize },

    Round { src: usize, dst: usize },
    Floor { src: usize, dst: usize },
    Ceil { src: usize, dst: usize },
    ToFloat { src: usize, dst: usize },

    Bool2Int { src: usize, dst: usize },
    Int2Bool { src: usize, dst: usize },

    Jmp { dst: usize },
    JmpIf { check: usize, dst: usize },
    Call { func: usize, args: Box<[usize]>, ret_locs: Box<[usize]> },
    Return { rets: Box<[usize]> },

    IOSetValue { offset: usize, src: usize },
    IOGetValue { offset: usize, dst: usize },
    CallFFI { func: usize, args: Box<[usize]>, ret_locs: Box<[usize]> },

    Yield
}


#[cfg(test)]
mod test {
    use crate::r25_300::insc::Insc;

    #[test]
    fn test() {
        dbg!(Insc::VARIANT_COUNT);
        dbg!(std::mem::size_of::<Insc>());
    }
}
