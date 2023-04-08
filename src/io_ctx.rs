use std::fmt::{Display, Formatter};
use xjbutil::void::Void;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Type21 {
    Int32 = 1,
    Float32 = 2,
    Bool = 3
}

impl Type21 {
    #[inline(always)] pub const fn size(&self) -> usize { 4 }
}

impl Display for Type21 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type21::Int32 => write!(f, "int"),
            Type21::Float32 => write!(f, "float"),
            Type21::Bool => write!(f, "bool")
        }
    }
}

pub trait Reflektor<T> {
    fn reflected_type() -> Type21;
}

impl Reflektor<i32> for Void {
    #[inline(always)] fn reflected_type() -> Type21 { Type21::Int32 }
}

impl Reflektor<f32> for Void {
    #[inline(always)] fn reflected_type() -> Type21 { Type21::Float32 }
}

pub type IOContextMetadata = Vec<(String, String, Type21)>;

pub trait IOContext {
    fn metadata() -> IOContextMetadata;
}
