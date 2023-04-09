use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use smallvec::{SmallVec, smallvec};

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
struct FunctionInfo {
    id: usize,
    args: Vec<Type21>,
    rets: Vec<Type21>
}

#[derive(Clone)]
pub struct CodegenContext<CTX: IOContext> {
    metadata: Rc<HashMap<String, (Type21, usize)>>,
    const_pool: HashMap<String, (Type21, RtValue)>,
    frame: Frame,

    compiled: Compiled,
    func: HashMap<String, FunctionInfo>,
    ffi_func: HashMap<String, FunctionInfo>,
    current_func: Option<FunctionInfo>,

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
            func: HashMap::new(),
            ffi_func: HashMap::new(),
            current_func: None,

            _ctx: PhantomData
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ExprValue {
    StackAddr(Type21, usize),
    ConstEval(Type21, RtValue)
}

impl ExprValue {
    pub fn type21(&self) -> Type21 {
        match self {
            ExprValue::StackAddr(ty, _) => *ty,
            ExprValue::ConstEval(ty, _) => *ty
        }
    }
}

impl<CTX: IOContext> CodegenContext<CTX> {
    fn ensure_addr(&mut self, v: ExprValue) -> usize {
        match v {
            ExprValue::StackAddr(_, addr) => addr,
            ExprValue::ConstEval(_, value) => {
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

macro_rules! impl_arithmetic_binop_constfold {
    ($ty:expr, $lhs:expr, $rhs:expr, $output_ty:expr, $op:tt) => {
        match $ty {
            Type21::Int32 => Ok(smallvec![ExprValue::ConstEval(
                $output_ty,
                unsafe { $lhs.i $op $rhs.i }.into()
            )]),
            Type21::Float32 => Ok(smallvec![ExprValue::ConstEval(
                $output_ty,
                unsafe { $lhs.f $op $rhs.f }.into()
            )]),
            _ => Err(format!("unsupported type for arithmetic binop: {:?}", $ty))
        }
    }
}

macro_rules! impl_logic_binop_constfold {
    ($ty:expr, $lhs:expr, $rhs:expr, $op:tt) => {
        match $ty {
            Type21::Bool => Ok(smallvec![ExprValue::ConstEval(
                Type21::Bool,
                unsafe { $lhs.b $op $rhs.b }.into()
            )]),
            _ => Err(format!("unsupported type for logic binop: {:?}", $ty))
        }
    }
}

macro_rules! impl_arithmetic_binop {
    ($this:expr, $ty:expr, $lhs:expr, $rhs:expr, $output_ty:expr, $dst: expr, $int_insc:ident, $float_insc:ident) => {
        match $ty {
            Type21::Int32 => {
                $this.compiled.code.push(Insc::$int_insc { lhs: $lhs, rhs: $rhs, dst: $dst });
                Ok(smallvec![ExprValue::StackAddr($output_ty, $dst)])
            },
            Type21::Float32 => {
                $this.compiled.code.push(Insc::$float_insc { lhs: $lhs, rhs: $rhs, dst: $dst });
                Ok(smallvec![ExprValue::StackAddr($output_ty, $dst)])
            },
            _ => Err(format!("unsupported type for arithmetic binop: {:?}", $ty))
        }
    }
}

macro_rules! impl_logic_binop {
    ($this:expr, $ty:expr, $lhs:expr, $rhs:expr, $dst: expr, $insc:ident) => {
        match $ty {
            Type21::Bool => {
                $this.compiled.code.push(Insc::$insc { lhs: $lhs, rhs: $rhs, dst: $dst });
                Ok(smallvec![ExprValue::StackAddr(Type21::Bool, $dst)])
            },
            _ => Err(format!("unsupported type for logic binop: {:?}", $ty))
        }
    }
}

impl<CTX: IOContext> SyntaxVisitor for CodegenContext<CTX> {
    type ExprResult = SmallVec<[ExprValue; 2]>;
    type StmtResult = ();
    type DeclResult = ();
    type Error = String;

    fn visit_ident(&mut self, ident: &str) -> Result<Self::ExprResult, Self::Error> {
        if let Some((addr, ty)) = self.frame.get_var(ident) {
            Ok(smallvec![ExprValue::StackAddr(ty, addr)])
        } else if let Some((ty, value)) = self.const_pool.get(ident) {
            Ok(smallvec![ExprValue::ConstEval(*ty, *value)])
        } else if let Some((ty, offset)) = self.metadata.get(ident) {
            let addr = self.frame.allocate();
            self.compiled.code.push(Insc::IOGetValue { offset: *offset, dst: addr });
            Ok(smallvec![ExprValue::StackAddr(*ty, addr)])
        } else {
            Err(format!("unknown identifier: {}", ident))
        }
    }

    fn visit_lit_int(&mut self, value: i32) -> Self::ExprResult {
        smallvec![ExprValue::ConstEval(Type21::Int32, value.into())]
    }

    fn visit_lit_float(&mut self, value: f32) -> Self::ExprResult {
        smallvec![ExprValue::ConstEval(Type21::Float32, value.into())]
    }

    fn visit_lit_bool(&mut self, value: bool) -> Self::ExprResult {
        smallvec![ExprValue::ConstEval(Type21::Bool, value.into())]
    }

    fn visit_uop(
        &mut self,
        op: UnaryOp,
        operand: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        if operand.len() >= 2 {
            return Err("cannot apply unary operator to value bundle".to_string());
        } else if operand.len() == 0 {
            return Err("cannot apply unary operator to nikhil".to_string());
        }

        let operand = operand[0];

        match op {
            UnaryOp::Negate => {
                match operand {
                    ExprValue::StackAddr(ty, addr) => {
                        match ty {
                            Type21::Int32 => {
                                let new_addr = self.frame.allocate();
                                self.compiled.code.push(Insc::NegateInt {
                                    src: addr, dst: new_addr
                                });
                                Ok(smallvec![ExprValue::StackAddr(ty, new_addr)])
                            },
                            Type21::Float32 => {
                                let new_addr = self.frame.allocate();
                                self.compiled.code.push(Insc::NegateFloat {
                                    src: addr, dst: new_addr
                                });
                                Ok(smallvec![ExprValue::StackAddr(ty, new_addr)])
                            },
                            Type21::Bool => Err("cannot negate a boolean".to_string())
                        }
                    }
                    ExprValue::ConstEval(ty, value) => {
                        match ty {
                            Type21::Int32 =>
                                Ok(smallvec![ExprValue::ConstEval(ty, unsafe { -value.i }.into())]),
                            Type21::Float32 =>
                                Ok(smallvec![ExprValue::ConstEval(ty, unsafe { -value.f }.into())]),
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
                    ExprValue::StackAddr(ty, addr) => {
                        let new_addr = self.frame.allocate();
                        self.compiled.code.push(Insc::Not {
                            src: addr, dst: new_addr
                        });
                        Ok(smallvec![ExprValue::StackAddr(ty, new_addr)])
                    }
                    ExprValue::ConstEval(ty, value) => {
                        Ok(smallvec![ExprValue::ConstEval(ty, unsafe { !value.b }.into())])
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
        if lhs.len() >= 2 || rhs.len() >= 2 {
            return Err("cannot apply binary operator to value bundle".to_string());
        } else if lhs.len() == 0 || rhs.len() == 0 {
            return Err("cannot apply binary operator to nikhil".to_string());
        }

        let lhs = lhs[0];
        let rhs = rhs[0];

        if lhs.type21() != rhs.type21() {
            return Err("cannot perform binary operation on operands of different types".to_string())
        }

        if let (ExprValue::ConstEval(ty, lhs_value),
                ExprValue::ConstEval(_, rhs_value)) = (lhs, rhs) {
            match op {
                BinaryOp::Add =>
                    impl_arithmetic_binop_constfold!(ty, lhs_value, rhs_value, ty, +),
                BinaryOp::Sub =>
                    impl_arithmetic_binop_constfold!(ty, lhs_value, rhs_value, ty, -),
                BinaryOp::Mul =>
                    impl_arithmetic_binop_constfold!(ty, lhs_value, rhs_value, ty, *),
                BinaryOp::Div =>
                    impl_arithmetic_binop_constfold!(ty, lhs_value, rhs_value, ty, /),
                BinaryOp::Mod => if let Type21::Int32 = ty {
                    Ok(smallvec![ExprValue::ConstEval(
                        ty, unsafe { lhs_value.i % rhs_value.i }.into())
                    ])
                } else {
                    Err("cannot perform modulo on non-integer types".to_string())
                },
                BinaryOp::Eq => Ok(smallvec![ExprValue::ConstEval(
                    Type21::Bool,
                    (lhs_value == rhs_value).into()
                )]),
                BinaryOp::Ne => Ok(smallvec![ExprValue::ConstEval(
                    Type21::Bool,
                    (lhs_value != rhs_value).into()
                )]),
                BinaryOp::Lt =>
                    impl_arithmetic_binop_constfold!(ty, lhs_value, rhs_value, Type21::Bool, <),
                BinaryOp::Le =>
                    impl_arithmetic_binop_constfold!(ty, lhs_value, rhs_value, Type21::Bool, <=),
                BinaryOp::Gt =>
                    impl_arithmetic_binop_constfold!(ty, lhs_value, rhs_value, Type21::Bool, >),
                BinaryOp::Ge =>
                    impl_arithmetic_binop_constfold!(ty, lhs_value, rhs_value, Type21::Bool, >=),
                BinaryOp::And =>
                    impl_logic_binop_constfold!(ty, lhs_value, rhs_value, &&),
                BinaryOp::Or =>
                    impl_logic_binop_constfold!(ty, lhs_value, rhs_value, ||),
            }
        } else {
            let ty = lhs.type21();
            let lhs_addr = self.ensure_addr(lhs);
            let rhs_addr = self.ensure_addr(rhs);
            let new_addr = self.frame.allocate();

            match op {
                BinaryOp::Add => impl_arithmetic_binop!(
                    self, ty, lhs_addr, rhs_addr, ty, new_addr, AddInt, AddFloat
                ),
                BinaryOp::Sub => impl_arithmetic_binop!(
                    self, ty, lhs_addr, rhs_addr, ty, new_addr, SubInt, SubFloat
                ),
                BinaryOp::Mul => impl_arithmetic_binop!(
                    self, ty, lhs_addr, rhs_addr, ty, new_addr, MulInt, MulFloat
                ),
                BinaryOp::Div => impl_arithmetic_binop!(
                    self, ty, lhs_addr, rhs_addr, ty, new_addr, DivInt, DivFloat
                ),
                BinaryOp::Mod => if let Type21::Int32 = ty {
                    self.compiled.code.push(Insc::ModInt {
                        lhs: lhs_addr, rhs: rhs_addr, dst: new_addr
                    });
                    Ok(smallvec![ExprValue::StackAddr(ty, new_addr)])
                } else {
                    Err("cannot perform modulo on non-integer types".to_string())
                },
                BinaryOp::Eq => {
                    self.compiled.code.push(Insc::Eq {
                        lhs: lhs_addr, rhs: rhs_addr, dst: new_addr
                    });
                    Ok(smallvec![ExprValue::StackAddr(Type21::Bool, new_addr)])
                },
                BinaryOp::Ne => {
                    self.compiled.code.push(Insc::Ne {
                        lhs: lhs_addr, rhs: rhs_addr, dst: new_addr
                    });
                    Ok(smallvec![ExprValue::StackAddr(Type21::Bool, new_addr)])
                },
                BinaryOp::Lt => impl_arithmetic_binop!(
                    self, ty, lhs_addr, rhs_addr, Type21::Bool, new_addr, LtInt, LtFloat
                ),
                BinaryOp::Le => impl_arithmetic_binop!(
                    self, ty, lhs_addr, rhs_addr, Type21::Bool, new_addr, LeInt, LeFloat
                ),
                BinaryOp::Gt => impl_arithmetic_binop!(
                    self, ty, rhs_addr, lhs_addr, Type21::Bool, new_addr, LtInt, LtFloat
                ),
                BinaryOp::Ge => impl_arithmetic_binop!(
                    self, ty, rhs_addr, lhs_addr, Type21::Bool, new_addr, LeInt, LeFloat
                ),
                BinaryOp::And => impl_logic_binop!(self, ty, lhs_addr, rhs_addr, new_addr, And),
                BinaryOp::Or => impl_logic_binop!(self, ty, lhs_addr, rhs_addr, new_addr, Or),
            }
        }
    }

    fn visit_assign(
        &mut self,
        name: &str,
        value: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        if value.len() >= 2 {
            return Err("cannot assign value bundle to single variable".to_string());
        } else if value.is_empty() {
            return Err("cannot assign empty value bundle to single variable".to_string());
        }

        let value = value[0];

        let ty = value.type21();

        if let Some((addr, var_type)) = self.frame.get_var(name) {
            if var_type != ty {
                return Err(format!(
                    "cannot assign value of type '{}' to variable '{}' of type '{}'",
                    ty,
                    name,
                    var_type
                ));
            }
            match value {
                ExprValue::ConstEval(_, value) => {
                    self.compiled.code.push(Insc::Const { value, dst: addr });
                }
                ExprValue::StackAddr(_, value_addr) => {
                    self.compiled.code.push(Insc::Dup { src: value_addr, dst: addr });
                }
            }
            Ok(smallvec![])
        } else if let Some((ioctx_var_type, offset)) = self.metadata.get(name) {
            if *ioctx_var_type != ty {
                return Err(format!(
                    "cannot assign value of type '{}' to ioctx variable '{}' of type '{}'",
                    ty,
                    name,
                    ioctx_var_type
                ));
            }

            let offset = *offset;
            let value_addr = self.ensure_addr(value);
            self.compiled.code.push(Insc::IOSetValue {
                offset,
                src: value_addr,
            });
            Ok(smallvec![])
        } else {
            return Err(format!("variable '{}' is not declared", name));
        }
    }

    fn visit_assign2(
        &mut self,
        names: &[&str],
        value: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        if names.len() != value.len() {
            return Err(format!(
                "cannot assign value bundle of size {} to {} variables",
                value.len(),
                names.len()
            ));
        }

        if names.len() == 0 {
            return Err("cannot assign empty value bundle to empty variable list".to_string());
        }

        for (name, value) in names.iter().zip(value.iter()) {
            let ty = value.type21();

            if let Some((addr, var_type)) = self.frame.get_var(name) {
                if var_type != ty {
                    return Err(format!(
                        "cannot assign value of type '{}' to variable '{}' of type '{}'",
                        ty,
                        name,
                        var_type
                    ));
                }
                match value {
                    ExprValue::ConstEval(_, value) => {
                        self.compiled.code.push(Insc::Const { value: *value, dst: addr });
                    }
                    ExprValue::StackAddr(_, value_addr) => {
                        self.compiled.code.push(Insc::Dup { src: *value_addr, dst: addr });
                    }
                }
            } else if let Some((ioctx_var_type, offset)) = self.metadata.get(*name) {
                if *ioctx_var_type != ty {
                    return Err(format!(
                        "cannot assign value of type '{}' to ioctx variable '{}' of type '{}'",
                        ty,
                        name,
                        ioctx_var_type
                    ));
                }

                let offset = *offset;
                let value_addr = self.ensure_addr(*value);
                self.compiled.code.push(Insc::IOSetValue {
                    offset,
                    src: value_addr,
                });
            } else {
                return Err(format!("variable '{}' is not declared", name));
            }
        }

        Ok(smallvec![])
    }

    fn visit_type_cast(
        &mut self,
        ty: Type21,
        operand: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error> {
        if operand.len() != 1 {
            return Err("cannot cast value bundle".to_string());
        }

        let operand = operand[0];

        let operand_ty = operand.type21();

        if operand_ty == ty {
            return Ok(smallvec![operand]);
        }

        let operand_addr = self.ensure_addr(operand);

        let new_addr = self.frame.allocate();

        match (operand_ty, ty) {
            (Type21::Int32, Type21::Float32) => {
                self.compiled.code.push(Insc::ToFloat {
                    src: operand_addr,
                    dst: new_addr,
                });
            }
            (Type21::Float32, Type21::Int32) => {
                self.compiled.code.push(Insc::Round {
                    src: operand_addr,
                    dst: new_addr,// lookup in compiled functions
                });
            }
            _ => {
                return Err(format!(
                    "cannot cast value of type '{}' to type '{}'",
                    operand_ty, ty
                ));
            }
        }

        Ok(smallvec![ExprValue::StackAddr(ty, new_addr)])
    }

    fn visit_call(
        &mut self,
        name: &str,
        args: &[Self::ExprResult]
    ) -> Result<Self::ExprResult, Self::Error> {
        let mut args = args.iter().flatten().collect::<Vec<_>>();

        if name == "floor" {
            if args.len() != 1 {
                return Err("floor() takes exactly one argument".to_string());
            }

            let arg = args[0];

            let arg_ty = arg.type21();

            if arg_ty != Type21::Float32 {
                return Err(format!("floor() argument must be of type 'float', not '{}'", arg_ty));
            }

            let arg_addr = self.ensure_addr(*arg);
            let new_addr = self.frame.allocate();
            self.compiled.code.push(Insc::Floor {
                src: arg_addr,
                dst: new_addr,
            });

            return Ok(smallvec![ExprValue::StackAddr(Type21::Int32, new_addr)]);
        }
        else if name == "ceil" {
            if args.len() != 1 {
                return Err("ceil() takes exactly one argument".to_string());
            }

            let arg = args[0];

            let arg_ty = arg.type21();

            if arg_ty != Type21::Float32 {
                return Err(format!("ceil() argument must be of type 'float', not '{}'", arg_ty));
            }

            let arg_addr = self.ensure_addr(*arg);
            let new_addr = self.frame.allocate();
            self.compiled.code.push(Insc::Ceil {
                src: arg_addr,
                dst: new_addr,
            });

            return Ok(smallvec![ExprValue::StackAddr(Type21::Int32, new_addr)]);
        }
        else if name == "round" {
            if args.len() != 1 {
                return Err("round() takes exactly one argument".to_string());
            }

            let arg = args[0];

            let arg_ty = arg.type21();

            if arg_ty != Type21::Float32 {
                return Err(format!("round() argument must be of type 'float', not '{}'", arg_ty));
            }

            let arg_addr = self.ensure_addr(*arg);
            let new_addr = self.frame.allocate();
            self.compiled.code.push(Insc::Round {
                src: arg_addr,
                dst: new_addr,
            });

            return Ok(smallvec![ExprValue::StackAddr(Type21::Int32, new_addr)]);
        }

        // look up in compiled functions
        if let Some(func_info) = self.func.get(name) {
            let func_info = func_info.clone();
            if args.len() != func_info.args.len() {
                return Err(format!(
                    "function '{}' takes {} arguments, not {}",
                    name,
                    func_info.args.len(),
                    args.len()
                ));
            }

            for (arg, arg_ty) in args.iter().zip(func_info.args.iter()) {
                if arg.type21() != *arg_ty {
                    return Err(format!(
                        "cannot pass value of type '{}' to function '{}' argument of type '{}'",
                        arg.type21(),
                        name,
                        arg_ty
                    ));
                }
            }

            let mut arg_addrs = Vec::with_capacity(args.len());
            for arg in args {
                arg_addrs.push(self.ensure_addr(*arg));
            }

            return Ok(if func_info.rets.len() == 0 {
                self.compiled.code.push(Insc::Call {
                    func: func_info.id,
                    args: arg_addrs.into_boxed_slice(),
                    ret_locs: Box::new([]),
                });

                smallvec![]
            } else {
                let mut ret_addrs = Vec::with_capacity(func_info.rets.len());
                let mut rets = Vec::with_capacity(func_info.rets.len());
                for ret_ty in func_info.rets.iter() {
                    let ret_addr = self.frame.allocate();
                    ret_addrs.push(ret_addr);
                    rets.push(ExprValue::StackAddr(*ret_ty, ret_addr));
                }

                self.compiled.code.push(Insc::Call {
                    func: func_info.id,
                    args: arg_addrs.into_boxed_slice(),
                    ret_locs: ret_addrs.into_boxed_slice(),
                });

                rets.into()
            })
        }

        // look up in FFI functions
        if let Some(ffi_func) = self.ffi_func.get(name) {
            let ffi_func = ffi_func.clone();
            if args.len() != ffi_func.args.len() {
                return Err(format!(
                    "function '{}' takes {} arguments, not {}",
                    name,
                    ffi_func.args.len(),
                    args.len()
                ));
            }

            for (arg, arg_ty) in args.iter().zip(ffi_func.args.iter()) {
                if arg.type21() != *arg_ty {
                    return Err(format!(
                        "cannot pass value of type '{}' to function '{}' argument of type '{}'",
                        arg.type21(),
                        name,
                        arg_ty
                    ));
                }
            }

            let mut arg_addrs = Vec::with_capacity(args.len());
            for arg in args {
                arg_addrs.push(self.ensure_addr(*arg));
            }

            return Ok(if ffi_func.rets.len() == 0 {
                self.compiled.code.push(Insc::CallFFI {
                    func: ffi_func.id,
                    args: arg_addrs.into_boxed_slice(),
                    ret_locs: Box::new([]),
                });

                smallvec![]
            } else {
                let mut ret_addrs = Vec::with_capacity(ffi_func.rets.len());
                let mut rets = Vec::with_capacity(ffi_func.rets.len());
                for ret_ty in ffi_func.rets.iter() {
                    let ret_addr = self.frame.allocate();
                    ret_addrs.push(ret_addr);
                    rets.push(ExprValue::StackAddr(*ret_ty, ret_addr));
                }

                self.compiled.code.push(Insc::CallFFI {
                    func: ffi_func.id,
                    args: arg_addrs.into_boxed_slice(),
                    ret_locs: ret_addrs.into_boxed_slice(),
                });

                rets.into()
            })
        }

        Err(format!("unknown function '{}'", name))
    }

    fn visit_expr_stmt(&mut self, _expr: Self::ExprResult) -> Self::StmtResult {}

    fn visit_decl_stmt(
        &mut self,
        _decl: Self::DeclResult
    ) -> Result<Self::StmtResult, Self::Error> {
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        cond: Self::ExprResult,
        then: Self::StmtResult,
        otherwise: Option<Self::StmtResult>
    ) -> Result<Self::StmtResult, Self::Error> {
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
