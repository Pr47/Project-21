use xjbutil::zvec::ZeroVec;
use crate::io_ctx::IOContext;
use crate::r25_300::compiled::Compiled;
use crate::r25_300::insc::Insc;
use crate::r25_300::stack::Stack;
use crate::r25_300::value::RtValue;

macro_rules! impl_binop {
    ($f:ident, $s:expr, $cf:expr, $lhs:expr, $rhs:expr, $dst:expr, $op:tt) => {
        {
            let lhs = $cf.get_value($s, *$lhs).$f;
            let rhs = $cf.get_value($s, *$rhs).$f;
            $cf.set_value($s, *$dst, RtValue::from(lhs $op rhs));
        }
    }
}

macro_rules! impl_uop {
    ($f:ident, $s:expr, $cf:expr, $src:expr, $dst:expr, $op:tt) => {
        {
            let src = $cf.get_value($s, *$src).$f;
            $cf.set_value($s, *$dst, RtValue::from($op src));
        }
    }
}

macro_rules! impl_uop_fn {
    ($f:ident, $s:expr, $cf:expr, $src:expr, $dst:expr, $op:ident) => {
        {
            let src = $cf.get_value($s, *$src).$f;
            $cf.set_value($s, *$dst, RtValue::from(src.$op()));
        }
    }
}

pub struct Combustor<'a, 'ctx, CTX: IOContext> {
    pub io_ctx: &'ctx mut CTX,

    stack: Stack<'a>,
    out_buf: ZeroVec<RtValue>,
    in_buf: ZeroVec<RtValue>,
}

impl<'a, 'ctx, CTX> Combustor<'a, 'ctx, CTX>
    where CTX: IOContext
{
    pub fn new(io_ctx: &'ctx mut CTX) -> Self {
        Self {
            io_ctx,

            stack: Stack::new(),
            out_buf: ZeroVec::with_capacity(8),
            in_buf: ZeroVec::with_capacity(8)
        }
    }

    pub unsafe fn combust(
        &mut self,
        compiled: &Compiled<'a>,
        entry: usize
    ) -> Option<usize> {
        let entry_fn = *compiled.func.get_unchecked(entry);
        self.stack.enter_frame(entry_fn.frame_size);
        self.combust_resume(compiled, entry_fn.addr)
    }

    pub unsafe fn combust_resume(
        &mut self,
        compiled: &Compiled<'a>,
        mut insc_ptr: usize
    ) -> Option<usize> {
        let mut current_frame = self.stack.last_frame();

        loop {
            match unsafe { compiled.code.get_unchecked(insc_ptr) } {
                Insc::IntConst { value, dst } =>
                    current_frame.set_value(&mut self.stack, *dst, RtValue::from(*value)),
                Insc::FloatConst { value, dst } =>
                    current_frame.set_value(&mut self.stack, *dst, RtValue::from(*value)),
                Insc::AddInt { lhs, rhs, dst } =>
                    impl_binop!(i, &mut self.stack, current_frame, lhs, rhs, dst, +),
                Insc::AddFloat { lhs, rhs, dst } =>
                    impl_binop!(f, &mut self.stack, current_frame, lhs, rhs, dst, +),
                Insc::SubInt { lhs, rhs, dst } =>
                    impl_binop!(i, &mut self.stack, current_frame, lhs, rhs, dst, -),
                Insc::SubFloat { lhs, rhs, dst } =>
                    impl_binop!(f, &mut self.stack, current_frame, lhs, rhs, dst, -),
                Insc::MulInt { lhs, rhs, dst } =>
                    impl_binop!(i, &mut self.stack, current_frame, lhs, rhs, dst, *),
                Insc::MulFloat { lhs, rhs, dst } =>
                    impl_binop!(f, &mut self.stack, current_frame, lhs, rhs, dst, *),
                Insc::DivInt { lhs, rhs, dst } =>
                    impl_binop!(i, &mut self.stack, current_frame, lhs, rhs, dst, /),
                Insc::DivFloat { lhs, rhs, dst } =>
                    impl_binop!(f, &mut self.stack, current_frame, lhs, rhs, dst, /),
                Insc::ModInt { lhs, rhs, dst } =>
                    impl_binop!(i, &mut self.stack, current_frame, lhs, rhs, dst, %),
                Insc::NegateInt { src, dst } =>
                    impl_uop!(i, &mut self.stack, current_frame, src, dst, -),
                Insc::NegateFloat { src, dst } =>
                    impl_uop!(f, &mut self.stack, current_frame, src, dst, -),
                Insc::Eq { lhs, rhs, dst } =>
                    impl_binop!(repr, &mut self.stack, current_frame, lhs, rhs, dst, ==),
                Insc::Neq { lhs, rhs, dst } =>
                    impl_binop!(repr, &mut self.stack, current_frame, lhs, rhs, dst, !=),
                Insc::LtInt { lhs, rhs, dst } =>
                    impl_binop!(i, &mut self.stack, current_frame, lhs, rhs, dst, <),
                Insc::LtFloat { lhs, rhs, dst } =>
                    impl_binop!(f, &mut self.stack, current_frame, lhs, rhs, dst, <),
                Insc::LeInt { lhs, rhs, dst } =>
                    impl_binop!(i, &mut self.stack, current_frame, lhs, rhs, dst, <=),
                Insc::LeFloat { lhs, rhs, dst } =>
                    impl_binop!(f, &mut self.stack, current_frame, lhs, rhs, dst, <=),
                Insc::And { lhs, rhs, dst } =>
                    impl_binop!(b, &mut self.stack, current_frame, lhs, rhs, dst, &&),
                Insc::Or { lhs, rhs, dst } =>
                    impl_binop!(b, &mut self.stack, current_frame, lhs, rhs, dst, ||),
                Insc::Not { src, dst } =>
                    impl_uop!(b, &mut self.stack, current_frame, src, dst, !),
                Insc::Round { src, dst } =>
                    impl_uop_fn!(f, &mut self.stack, current_frame, src, dst, round),
                Insc::Floor { src, dst } =>
                    impl_uop_fn!(f, &mut self.stack, current_frame, src, dst, floor),
                Insc::Ceil { src, dst } =>
                    impl_uop_fn!(f, &mut self.stack, current_frame, src, dst, ceil),
                Insc::ToFloat { src, dst } => {
                    let src = current_frame.get_value(&mut self.stack, *src).i;
                    current_frame.set_value(&mut self.stack, *dst, RtValue::from(src as f32));
                },
                Insc::Bool2Int { src, dst } => {
                    let src = current_frame.get_value(&mut self.stack, *src).b;
                    current_frame.set_value(&mut self.stack, *dst, RtValue::from(src as i32));
                },
                Insc::Int2Bool { src, dst } => {
                    let src = current_frame.get_value(&mut self.stack, *src).i;
                    current_frame.set_value(&mut self.stack, *dst, RtValue::from(src != 0));
                },
                Insc::Jmp { dst } => {
                    insc_ptr = *dst;
                    continue;
                },
                Insc::JmpIf { check, dst } => {
                    let check = current_frame.get_value(&mut self.stack, *check).b;
                    if check {
                        insc_ptr = *dst;
                        continue;
                    }
                },
                Insc::Call { func, args, ret_locs } => {
                    let func = *compiled.func.get_unchecked(*func);
                    current_frame = self.stack.call_enter_frame(
                        insc_ptr,
                        func.frame_size,
                        args,
                        ret_locs
                    );
                    insc_ptr = func.addr;
                    continue;
                },
                Insc::Return { rets } => {
                    if let Some((frame, ret_addr)) = self.stack.exit_frame(rets) {
                        current_frame = frame;
                        insc_ptr = ret_addr;
                    } else {
                        break;
                    }
                },
                Insc::IOSetValue { offset, src } => {
                    let src = current_frame.get_value(&mut self.stack, *src);
                    // just unsafely write memory to destination offset
                    (&mut self.io_ctx as *mut _ as *mut u8)
                        .add(*offset)
                        .write(&src as *const _ as _);
                },
                Insc::IOGetValue { offset, dst } => {
                    let src = ((&mut self.io_ctx as *const _ as *const u8)
                        .add(*offset)
                        as *const RtValue)
                        .read();
                    current_frame.set_value(&mut self.stack, *dst, src);
                },
                Insc::CallFFI { func, args, ret_locs } => {
                    let arg_count = args.len();
                    let ret_count = ret_locs.len();
                    self.in_buf.resize(arg_count);
                    self.out_buf.resize(ret_count);

                    for i in 0..arg_count {
                        let arg = current_frame.get_value(&mut self.stack, *args.get_unchecked(i));
                        *self.in_buf.get_unchecked_mut(i) = arg;
                    }

                    let func = *compiled.ffi.get_unchecked(*func);
                    (func)(
                        self.in_buf.get_unchecked_mut(0) as *mut _,
                        arg_count as u32,
                        self.out_buf.get_unchecked_mut(0) as *mut _
                    );

                    for i in 0..ret_count {
                        let ret = *self.out_buf.get_unchecked(i);
                        let dst = *ret_locs.get_unchecked(i);
                        current_frame.set_value(&mut self.stack, dst, ret);
                    }
                },
                Insc::Yield => {
                    return Some(insc_ptr + 1)
                }
            }
            insc_ptr += 1;
        }

        None
    }
}
