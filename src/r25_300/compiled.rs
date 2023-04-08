use crate::value::RawFunction;
use crate::r25_300::insc::Insc;

#[derive(Debug, Copy, Clone)]
pub struct Function {
    pub addr: usize,
    pub frame_size: usize
}

#[derive(Debug, Clone)]
pub struct Compiled {
    pub code: Vec<Insc>,
    pub func: Vec<Function>,
    pub ffi: Vec<RawFunction>
}

impl Compiled {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            func: Vec::new(),
            ffi: Vec::new()
        }
    }
}
