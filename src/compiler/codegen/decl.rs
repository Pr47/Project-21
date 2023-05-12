use std::collections::HashMap;
use smallvec::{SmallVec, smallvec};

use crate::compiler::codegen::CodegenContext;
use crate::compiler::parse::cst::*;

use crate::io_ctx::Type21;
use crate::r25_300::compiled::Function;
use crate::value::RtValue;

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

impl CompilingFunction {
    pub fn push_frame(&mut self, named_var_count: usize) {
        self.frames.push(FunctionFrame {
            anonymous_count: 0,
            named_vars: HashMap::new(),
            frame_start: self.stack_usage,
            named_var_count
        });
    }

    pub fn pop_frame(&mut self) {
        let last_frame = self.frames.pop().unwrap();
        self.stack_usage -= last_frame.anonymous_count + last_frame.named_vars.len();
    }

    pub fn try_add_var(&mut self, var_name: &str, ty: RtValue) -> Result<(), String> {
        let last_frame = self.frames.last().unwrap();

        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionFrame {
    pub anonymous_count: usize,
    pub named_vars: HashMap<String, VarInfo>,

    pub frame_start: usize,
    pub named_var_count: usize
}

impl CodegenContext {
    pub fn visit_func_decl(&mut self, func_decl: &FuncDecl) -> Result<(), String> {
        let Some(func_body) = &func_decl.body else {
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
                    )).collect::<_>(),
                    frame_start: 0,
                    named_var_count: func_decl.params.len()
                }
            ]
        });

        self.compiling_func.as_mut().unwrap().push_frame(self.count_named_var(&func_body.stmts));

        let start_addr = self.compiled.code.len();
        for stmt in func_body.stmts {
            self.codegen_stmt(&stmt)?;
        }
        let end_addr = self.compiled.code.len();

        self.compiled.func.push(Function {
            name: func_decl.name.clone(),
            addr: start_addr,
            frame_size: self.compiling_func.as_ref().unwrap().max_stack_usage,
            code_len: end_addr - start_addr
        });
        self.compiling_func = None;

        Ok(())
    }

    fn count_named_var(&self, stmts: &[Stmt]) -> usize {
        let mut count = 0;
        for stmt in stmts {
            if let Stmt::DeclStmt(_) = stmt {
                count += 1;
            }
        }
        count
    }

    pub fn visit_var_decl(&mut self, var_decl: &VarDecl) -> Result<(), String> {
        let compiling_func = self.compiling_func.as_mut().unwrap();
        let last_frame = compiling_func.frames.last_mut().unwrap();

        if last_frame.named_vars.get(&var_decl.name).is_some() {
            return Err(format!("行 {}: 重复的变量定义 `{}`", var_decl.line, var_decl.name));
        }

        if var_decl.ty.is_none() && var_decl.init.is_none() {
            return Err(format!(
                "行 {}: 必须初始化变量 `{}` 或为其指定类型",
                var_decl.line,
                var_decl.name
            ));
        }

        let var_info = VarInfo {
            loc: last_frame.named_vars.len(),
            ty: todo!()
        };
        last_frame.named_vars.insert(var_decl.name.clone(), var_info);

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
