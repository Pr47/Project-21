use std::collections::HashMap;
use smallvec::SmallVec;
use crate::compiler::codegen::CodegenContext;
use crate::compiler::parse::cst::FuncDecl;

use crate::io_ctx::Type21;

#[derive(Debug, Copy, Clone)]
pub struct VarInfo {
    pub loc: usize,
    pub ty: Type21
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub ty: SmallVec<[Type21; 2]>,
    pub params: SmallVec<[(Type21, String); 2]>,

    pub start_addr: Option<usize>
}

#[derive(Debug, Clone)]
pub struct CompilingFunction {
    pub func_info: FunctionInfo,

    pub stack_usage: usize,
    pub frames: SmallVec<[FunctionFrame; 2]>
}

#[derive(Debug, Clone)]
pub struct FunctionFrame {
    pub anonymous_count: usize,
    pub named_vars: HashMap<String, VarInfo>
}

impl CodegenContext {
    pub fn visit_func_decl(&mut self, func_decl: &FuncDecl) -> Result<(), String> {
        let Some(_func_body) = &func_decl.body else {
            self.declared_func.insert(
                func_decl.name.to_string(),
                FunctionInfo {
                    ty: func_decl.ty.clone(),
                    params: func_decl.params.clone(),
                    start_addr: None,
                }
            );
            return Ok(());
        };

        todo!()
    }
}
