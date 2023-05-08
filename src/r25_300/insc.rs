use std::fmt::{Display, Formatter};
use crate::value::RtValue;
#[cfg(test)] use variant_count::VariantCount;

#[cfg_attr(test, derive(VariantCount))]
#[derive(Debug, Clone)]
pub enum Insc {
    Const { value: RtValue, dst: usize },
    Dup { src: usize, dst: usize },

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

impl Display for Insc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Insc::Const { value, dst } => writeln!(f, "mov ${:X}, %{}", unsafe { value.repr }, dst),
            Insc::Dup { src, dst } => writeln!(f, "mov ${}, %{}", src, dst),

            Insc::AddInt { lhs, rhs, dst } => writeln!(f, "add %{}, %{}, %{}", lhs, rhs, dst),
            Insc::AddFloat { lhs, rhs, dst } => writeln!(f, "fadd %{}, %{}, %{}", lhs, rhs, dst),
            Insc::SubInt { lhs, rhs, dst } => writeln!(f, "sub %{}, %{}, %{}", lhs, rhs, dst),
            Insc::SubFloat { lhs, rhs, dst } => writeln!(f, "fsub %{}, %{}, %{}", lhs, rhs, dst),
            Insc::MulInt { lhs, rhs, dst } => writeln!(f, "mul %{}, %{}, %{}", lhs, rhs, dst),
            Insc::MulFloat { lhs, rhs, dst } => writeln!(f, "fmul %{}, %{}, %{}", lhs, rhs, dst),
            Insc::DivInt { lhs, rhs, dst } => writeln!(f, "div %{}, %{}, %{}", lhs, rhs, dst),
            Insc::DivFloat { lhs, rhs, dst } => writeln!(f, "fdiv %{}, %{}, %{}", lhs, rhs, dst),
            Insc::ModInt { lhs, rhs, dst } => writeln!(f, "mod %{}, %{}, %{}", lhs, rhs, dst),

            Insc::NegateInt { src, dst } => writeln!(f, "neg %{}, %{}", src, dst),
            Insc::NegateFloat { src, dst } => writeln!(f, "fneg %{}, %{}", src, dst),

            Insc::Eq { lhs, rhs, dst } => writeln!(f, "eq %{}, %{}, %{}", lhs, rhs, dst),
            Insc::Ne { lhs, rhs, dst } => writeln!(f, "ne %{}, %{}, %{}", lhs, rhs, dst),

            Insc::LtInt { lhs, rhs, dst } => writeln!(f, "lt %{}, %{}, %{}", lhs, rhs, dst),
            Insc::LtFloat { lhs, rhs, dst } => writeln!(f, "flt %{}, %{}, %{}", lhs, rhs, dst),
            Insc::LeInt { lhs, rhs, dst } => writeln!(f, "le %{}, %{}, %{}", lhs, rhs, dst),
            Insc::LeFloat { lhs, rhs, dst } => writeln!(f, "fle %{}, %{}, %{}", lhs, rhs, dst),

            Insc::And { lhs, rhs, dst } => writeln!(f, "and %{}, %{}, %{}", lhs, rhs, dst),
            Insc::Or { lhs, rhs, dst } => writeln!(f, "or %{}, %{}, %{}", lhs, rhs, dst),
            Insc::Not { src, dst } => writeln!(f, "not %{}, %{}", src, dst),

            Insc::Round { src, dst } => writeln!(f, "round %{}, %{}", src, dst),
            Insc::Floor { src, dst } => writeln!(f, "floor %{}, %{}", src, dst),
            Insc::Ceil { src, dst } => writeln!(f, "ceil %{}, %{}", src, dst),
            Insc::ToFloat { src, dst } => writeln!(f, "tofloat %{}, %{}", src, dst),

            Insc::Bool2Int { src, dst } => writeln!(f, "b2i %{}, %{}", src, dst),
            Insc::Int2Bool { src, dst } => writeln!(f, "i2b %{}, %{}", src, dst),

            Insc::Jmp { dst } => writeln!(f, "jmp {}", dst),
            Insc::JmpIf { check, dst } => writeln!(f, "jmpif %{}, {}", check, dst),
            Insc::Call { func, args, ret_locs } => {
                write!(f, "call @{}(", func)?;
                for (idx, arg) in args.iter().enumerate() {
                    write!(f, "%{}", arg)?;
                    if idx != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "; [")?;
                for (idx, ret) in ret_locs.iter().enumerate() {
                    write!(f, "%{}", ret)?;
                    if idx != ret_locs.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                writeln!(f, "])")
            }
            Insc::Return { rets } => {
                if rets.len() == 0 {
                    writeln!(f, "ret")
                } else if rets.len() == 1 {
                    writeln!(f, "ret %{}", rets[0])
                } else {
                    write!(f, "ret [")?;
                    for (idx, ret) in rets.iter().enumerate() {
                        write!(f, "%{}", ret)?;
                        if idx != rets.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    writeln!(f, "]")
                }
            },

            Insc::IOSetValue { offset, src } => writeln!(f, "ioset !{:X} %{}", offset, src),
            Insc::IOGetValue { offset, dst } => writeln!(f, "ioget !{:X} %{}", offset, dst),
            Insc::CallFFI { func, args, ret_locs } => {
                write!(f, "call-ffi @{}(", func)?;
                for (idx, arg) in args.iter().enumerate() {
                    write!(f, "%{}", arg)?;
                    if idx != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "; [")?;
                for (idx, ret) in ret_locs.iter().enumerate() {
                    write!(f, "%{}", ret)?;
                    if idx != ret_locs.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                writeln!(f, "])")
            }
            Insc::Yield => writeln!(f, "yield"),
        }
    }
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
