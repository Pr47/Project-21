#[derive(Copy, Clone)]
#[repr(C)]
pub union RtValue {
    pub i: i32,
    pub f: f32,
    pub b: bool,

    pub repr: u32
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

pub type RawFunction = unsafe fn(args: *mut RtValue, n_args: u32, rets: *mut RtValue);
