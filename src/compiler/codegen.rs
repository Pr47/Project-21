use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::compiler::lex::TokenData;
use crate::compiler::op::{BinaryOp, UnaryOp};
use crate::compiler::visit::SyntaxVisitor;
use crate::io_ctx::{IOContext, IOContextMetadata, Type21};
use crate::r25_300::compiled::Compiled;
use crate::r25_300::insc::Insc;
use crate::value::RtValue;

#[derive(Clone)]
struct FramePart {
    part_size: usize,
    mapped_vars: HashMap<String, (usize, Type21)>,
    mapped_const: HashMap<RtValue, usize>
}

#[derive(Clone)]
struct Frame {
    frame: Vec<FramePart>,
    frame_size: usize,
    frame_max_size: usize
}

impl Frame {
    pub fn new() -> Self {
        Self {
            frame: Vec::new(),
            frame_size: 0,
            frame_max_size: 0
        }
    }

    pub fn pop_frame_part(&mut self) {
        let part = self.frame.pop().unwrap();
        self.frame_size -= part.part_size;
    }

    pub fn push_frame_part(&mut self, init_part_size: usize) {
        self.frame.push(FramePart {
            part_size: init_part_size,
            mapped_vars: HashMap::new(),
            mapped_const: HashMap::new()
        });
        self.frame_size += init_part_size;

        self.frame_max_size = self.frame_max_size.max(self.frame_size);
    }

    pub fn clear_frame(&mut self) {
        self.frame.clear();
        self.frame_size = 0;
    }

    pub fn allocate(&mut self) -> usize {
        let addr = self.frame_size;
        self.frame_size += 1;
        self.frame_max_size = self.frame_max_size.max(self.frame_size);
        addr
    }

    pub fn push_var(&mut self, name: &str, ty: Type21) -> usize {
        let addr = self.allocate();
        let part = self.frame.last_mut().unwrap();
        part.mapped_vars.insert(name.to_string(), (addr, ty));
        addr
    }

    pub fn push_const(&mut self, value: RtValue) -> usize {
        let addr = self.allocate();
        let part = self.frame.last_mut().unwrap();
        part.mapped_const.insert(value, addr);
        addr
    }

    pub fn get_var(&self, name: &str) -> Option<(usize, Type21)> {
        for part in self.frame.iter().rev() {
            if let Some(var) = part.mapped_vars.get(name) {
                return Some(*var);
            }
        }
        None
    }

    pub fn get_const(&self, value: RtValue) -> Option<usize> {
        for part in self.frame.iter().rev() {
            if let Some(addr) = part.mapped_const.get(&value) {
                return Some(*addr);
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct CodegenContext<CTX: IOContext> {
    metadata: Rc<HashMap<String, (Type21, usize)>>,
    const_pool: HashMap<String, (Type21, RtValue)>,
    frame: Frame,

    compiled: Compiled,

    _ctx: PhantomData<CTX>
}

impl<CTX: IOContext> CodegenContext<CTX> {
    pub fn new() -> Self {
        unsafe { Self::with_unproven_metadata(IOContextMetadata::new()) }
    }

    pub unsafe fn with_unproven_metadata(metadata: IOContextMetadata) -> Self {
        let mut transformed_metadata = HashMap::new();
        let mut current_offset = 0;

        for (mapped_name, _, ty) in metadata {
            transformed_metadata.insert(mapped_name, (ty, current_offset));
            current_offset += ty.size();
        }

        Self {
            metadata: Rc::new(transformed_metadata),
            const_pool: HashMap::new(),
            frame: Frame::new(),

            compiled: Compiled::new(),

            _ctx: PhantomData
        }
    }
}

#[derive(Copy, Clone)]
pub enum ExprResult {
    StackAddr(Type21, usize),
    ConstEval(Type21, RtValue)
}

impl ExprResult {
    pub fn type21(&self) -> Type21 {
        match self {
            ExprResult::StackAddr(ty, _) => *ty,
            ExprResult::ConstEval(ty, _) => *ty
        }
    }
}

impl<CTX: IOContext> CodegenContext<CTX> {
    fn ensure_stable_addr(&mut self, v: ExprResult) -> usize {
        match v {
            ExprResult::StackAddr(_, addr) => addr,
            ExprResult::ConstEval(_, value) => {
                if let Some(addr) = self.frame.get_const(value) {
                    addr
                } else {
                    let addr = self.frame.push_const(value);
                    self.compiled.code.push(Insc::Const { value, dst: addr });
                    addr
                }
            }
        }
    }
}

impl<CTX: IOContext> SyntaxVisitor for CodegenContext<CTX> {
    type ExprResult = ExprResult;
    type StmtResult = ();
    type DeclResult = ();
    type Error = String;

    fn visit_ident(&mut self, ident: &str) -> Result<Self::ExprResult, Self::Error> {
        if let Some((addr, ty)) = self.frame.get_var(ident) {
            Ok(ExprResult::StackAddr(ty, addr))
        } else if let Some((ty, value)) = self.const_pool.get(ident) {
            Ok(ExprResult::ConstEval(*ty, *value))
        } else if let Some((ty, offset)) = self.metadata.get(ident) {
            let addr = self.frame.allocate();
            self.compiled.code.push(Insc::IOGetValue { offset: *offset, dst: addr });
            Ok(ExprResult::StackAddr(*ty, addr))
        } else {
            Err(format!("unknown identifier: {}", ident))
        }
    }

    fn visit_lit_int(&mut self, value: i32) -> Self::ExprResult {
        ExprResult::ConstEval(Type21::Int32, value.into())
    }

    fn visit_lit_float(&mut self, value: f32) -> Self::ExprResult {
        ExprResult::ConstEval(Type21::Float32, value.into())
    }

    fn visit_lit_bool(&mut self, value: bool) -> Self::ExprResult {
        ExprResult::ConstEval(Type21::Bool, value.into())
    }

    fn visit_uop(
        &mut self,
        op: UnaryOp,
        operand: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        match op {
            UnaryOp::Negate => {
                match operand {
                    ExprResult::StackAddr(ty, addr) => {
                        match ty {
                            Type21::Int32 => {
                                let new_addr = self.frame.allocate();
                                self.compiled.code.push(Insc::NegateInt {
                                    src: addr, dst: new_addr
                                });
                                Ok(ExprResult::StackAddr(ty, new_addr))
                            },
                            Type21::Float32 => {
                                let new_addr = self.frame.allocate();
                                self.compiled.code.push(Insc::NegateFloat {
                                    src: addr, dst: new_addr
                                });
                                Ok(ExprResult::StackAddr(ty, new_addr))
                            },
                            Type21::Bool => Err("cannot negate a boolean".to_string())
                        }
                    }
                    ExprResult::ConstEval(ty, value) => {
                        match ty {
                            Type21::Int32 =>
                                Ok(ExprResult::ConstEval(ty, unsafe { -value.i }.into())),
                            Type21::Float32 =>
                                Ok(ExprResult::ConstEval(ty, unsafe { -value.f }.into())),
                            Type21::Bool => Err("cannot negate a boolean".to_string())
                        }
                    }
                }
            },
            UnaryOp::Not => {
                if operand.type21() != Type21::Bool {
                    return Err("cannot negate a non-boolean".to_string())
                }

                match operand {
                    ExprResult::StackAddr(ty, addr) => {
                        let new_addr = self.frame.allocate();
                        self.compiled.code.push(Insc::Not {
                            src: addr, dst: new_addr
                        });
                        Ok(ExprResult::StackAddr(ty, new_addr))
                    }
                    ExprResult::ConstEval(ty, value) => {
                        Ok(ExprResult::ConstEval(ty, unsafe { !value.b }.into()))
                    }
                }
            }
        }
    }

    fn visit_bin_op(
        &mut self,
        op: BinaryOp,
        lhs: Self::ExprResult,
        rhs: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        if lhs.type21() != rhs.type21() {
            return Err("cannot perform binary operation on operands of different types".to_string())
        }

        todo!()
    }

    fn visit_assign(&mut self, assign_op: &TokenData, names: &str, value: Self::ExprResult) -> Result<Self::ExprResult, Self::Error> {
        todo!()
    }

    fn visit_assign2(&mut self, names: &[&str], value: Self::ExprResult) -> Result<Self::ExprResult, Self::Error> {
        todo!()
    }

    fn visit_type_cast(&mut self, ty: Type21, operand: Self::ExprResult) -> Result<Self::ExprResult, Self::Error> {
        todo!()
    }

    fn visit_call(&mut self, name: &str, args: &[Self::ExprResult]) -> Result<Self::ExprResult, Self::Error> {
        todo!()
    }

    fn visit_expr_stmt(&mut self, expr: Self::ExprResult) -> Self::StmtResult {
        todo!()
    }

    fn visit_decl_stmt(&mut self, decl: Self::DeclResult) -> Result<Self::StmtResult, Self::Error> {
        todo!()
    }

    fn visit_if_stmt(&mut self, cond: Self::ExprResult, then: Self::StmtResult, otherwise: Option<Self::StmtResult>) -> Result<Self::StmtResult, Self::Error> {
        todo!()
    }

    fn visit_while_stmt(&mut self, cond: Self::ExprResult, body: Self::StmtResult) -> Result<Self::StmtResult, Self::Error> {
        todo!()
    }

    fn visit_for_stmt(&mut self, init: Option<Self::ExprResult>, cond: Option<Self::ExprResult>, step: Option<Self::ExprResult>, body: Self::StmtResult) -> Result<Self::StmtResult, Self::Error> {
        todo!()
    }

    fn visit_break_stmt(&mut self) -> Result<Self::StmtResult, Self::Error> {
        todo!()
    }

    fn visit_continue_stmt(&mut self) -> Result<Self::StmtResult, Self::Error> {
        todo!()
    }

    fn visit_return_stmt(&mut self, value: Option<Self::ExprResult>) -> Result<Self::StmtResult, Self::Error> {
        todo!()
    }

    fn visit_block_stmt(&mut self, stmts: &[Self::StmtResult]) -> Result<Self::StmtResult, Self::Error> {
        todo!()
    }

    fn visit_var_decl(&mut self, ty: Option<Type21>, name: &str, init: Option<Self::ExprResult>) -> Result<Self::DeclResult, Self::Error> {
        todo!()
    }

    fn visit_func_decl(&mut self, ty: &[Type21], name: &str, params: &[(Type21, &str)], body: Option<Self::StmtResult>) -> Result<Self::DeclResult, Self::Error> {
        todo!()
    }

    fn visit_const_decl(&mut self, name: &str, init: Self::ExprResult) -> Result<Self::DeclResult, Self::Error> {
        todo!()
    }
}
