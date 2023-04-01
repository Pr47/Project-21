use crate::r25_300::ffi::RawFunction;
use crate::r25_300::insc::Insc;

#[derive(Debug, Copy, Clone)]
pub struct Function {
    pub addr: usize,
    pub frame_size: usize
}

#[derive(Debug)]
pub struct Compiled<'a> {
    pub code: Vec<Insc<'a>>,
    pub func: Vec<Function>,
    pub ffi: Vec<RawFunction>
}
