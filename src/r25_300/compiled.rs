use std::fmt::{Display, Formatter};
use crate::value::RawFunction;
use crate::r25_300::insc::Insc;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub addr: usize,
    pub frame_size: usize,
    pub code_len: usize
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

impl Display for Compiled {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, func) in self.func.iter().enumerate() {
            writeln!(f, "{} ({}):", func.name, idx)?;
            for insc in self.code[func.addr..func.addr + func.code_len].iter() {
                writeln!(f, "  {}", insc)?;
            }
        }

        Ok(())
    }
}
