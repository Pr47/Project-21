use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
#[repr(C)]
pub union RtValue {
    pub i: i32,
    pub f: f32,
    pub b: bool,

    pub repr: u32
}

impl Debug for RtValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", unsafe { self.repr })
    }
}

impl From<i32> for RtValue {
    #[inline(always)] fn from(i: i32) -> Self {
        Self { i }
    }
}

impl From<f32> for RtValue {
    #[inline(always)] fn from(f: f32) -> Self {
        Self { f }
    }
}

impl From<bool> for RtValue {
    #[inline(always)] fn from(b: bool) -> Self {
        Self { b }
    }
}

impl PartialEq for RtValue {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.repr == other.repr }
    }
}

impl Eq for RtValue {}

impl Hash for RtValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { self.repr.hash(state); }
    }
}

pub type RawFunction = unsafe fn(args: *mut RtValue, n_args: u32, rets: *mut RtValue);
