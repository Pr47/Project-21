use std::collections::HashMap;
use smallvec::SmallVec;
use crate::io_ctx::Type21;

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub ty: SmallVec<[Type21; 2]>,
    pub params: SmallVec<[(Type21, String); 2]>,

    pub mangled_name: String,
    pub is_generator: bool,
    pub num_state: usize
}

#[derive(Debug, Clone)]
pub struct StateVar {
    pub mangled_name: String,
    pub ty: Type21
}

#[derive(Debug, Clone)]
pub struct CompilingFunction {
    pub func_info: FunctionInfo,

    pub context_vars: HashMap<String, StateVar>,
    pub state_vars: HashMap<String, StateVar>
}
