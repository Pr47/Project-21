use xjbutil::void::Void;

#[derive(Debug, Clone)]
pub enum Type21 {
    Int32 = 1,
    Float32 = 2
}

impl Type21 {
    #[inline(always)] pub const fn size(&self) -> usize { 4 }
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
