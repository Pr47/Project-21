use std::collections::HashMap;
use smallvec::{SmallVec, smallvec};

use crate::compiler::codegen::CodegenContext;
use crate::compiler::parse::cst::FuncDecl;

use crate::io_ctx::Type21;
use crate::r25_300::compiled::Function;

#[derive(Debug, Copy, Clone)]
pub struct VarInfo {
    pub loc: usize,
    pub ty: Type21
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub ty: SmallVec<[Type21; 2]>,
    pub params: SmallVec<[(Type21, String); 2]>,

    pub func_id: Option<usize>
}

impl From<&FuncDecl> for FunctionInfo {
    fn from(func_decl: &FuncDecl) -> Self {
        Self {
            ty: func_decl.ty.clone(),
            params: func_decl.params.clone(),
            func_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompilingFunction {
    pub func_info: FunctionInfo,

    pub stack_usage: usize,
    pub max_stack_usage: usize,
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
            return Ok(if let Some(prev_info) = self.declared_func.get(&func_decl.name) {
                Self::check_func_decl_coherence(func_decl, prev_info)?;
            } else {
                self.declared_func.insert(
                    func_decl.name.to_string(),
                    FunctionInfo::from(func_decl)
                );
            });
        };

        let func_info = if let Some(func_info) = self.declared_func.get_mut(&func_decl.name) {
            Self::check_func_decl_coherence(func_decl, func_info)?;

            if func_info.func_id.is_some() {
                return Err(format!(
                    "行 {}: 重复的函数定义 `{}`",
                    func_decl.line,
                    func_decl.name
                ));
            }

            func_info.func_id = Some(self.compiled.func.len());
            func_info.clone()
        } else {
            let mut func_info = FunctionInfo::from(func_decl);
            func_info.func_id = Some(self.compiled.func.len());
            self.declared_func.insert(func_decl.name.clone(), func_info.clone());
            func_info
        };

        self.compiling_func = Some(CompilingFunction {
            func_info,

            stack_usage: func_decl.params.len(),
            max_stack_usage: func_decl.params.len(),
            frames: smallvec![
                FunctionFrame {
                    anonymous_count: 0,
                    named_vars: func_decl.params.iter().enumerate().map(|(loc, (ty, name))| (
                        (name.clone(), VarInfo { loc, ty: *ty })
                    )).collect::<_>()
                },
                FunctionFrame {
                    anonymous_count: 0,
                    named_vars: HashMap::new()
                }
            ]
        });

        let start_addr = self.compiled.code.len();

        // TODO implement concrete logics here

        let end_addr = self.compiled.code.len();

        self.compiled.func.push(Function {
            name: func_decl.name.clone(),
            addr: start_addr,
            frame_size: self.compiling_func.unwrap().max_stack_usage,
            code_len: end_addr - start_addr
        });
        self.compiling_func = None;

        Ok(())
    }

    fn check_func_decl_coherence(decl: &FuncDecl, func_info: &FunctionInfo) -> Result<(), String> {
        if decl.params.len() != func_info.params.len() {
            return Err(format!(
                "行 {}: 函数 `{}` 先后以不同的参数个数被声明",
                decl.line,
                decl.name
            ));
        }

        Ok(())
    }
}
