use xjbutil::zvec::{ZeroVec, TrivialInit};

#[derive(Copy, Clone)]
#[repr(C)]
pub union RtValue {
    i: i32,
    f: f32,
    b: bool
}

unsafe impl TrivialInit for RtValue {}

#[derive(Copy, Clone)]
pub struct StackFrame<'a> {
    start_idx: usize,
    end_idx: usize,
    ret_locs: &'a [usize]
}

impl<'a> StackFrame<'a> {
    #[inline(always)]
    pub fn new(start_idx: usize, end_idx: usize, ret_locs: &'a [usize]) -> Self {
        Self {
            start_idx,
            end_idx,
            ret_locs
        }
    }
}

impl StackFrame<'_> {
    #[inline(always)]
    pub fn get_value<'a>(&'a self, stack: &'_ Stack<'_>, idx: usize) -> RtValue {
        *stack.values.get_unchecked(self.start_idx + idx)
    }

    #[inline(always)]
    pub fn set_value<'a>(&'a self, stack: &'_ mut Stack<'_>, idx: usize, value: RtValue) {
        *stack.values.get_unchecked_mut(self.start_idx + idx) = value;
    }
}

unsafe impl TrivialInit for StackFrame<'_> {}

pub struct Stack<'a> {
    values: ZeroVec<RtValue>,
    frames: Vec<StackFrame<'a>>
}

impl Stack<'_> {
    pub fn new() -> Self {
        Self {
            values: ZeroVec::new(),
            frames: Vec::new()
        }
    }
}

impl<'a> Stack<'a> {
    pub fn enter_frame(&mut self, frame_size: usize) -> StackFrame<'a> {
        debug_assert!(self.frames.is_empty());
        debug_assert!(self.values.is_empty());

        self.values.resize(frame_size);
        let frame = StackFrame::new(0, frame_size, &[]);
        self.frames.push(frame);
        frame
    }

    pub fn call_enter_frame(&mut self, frame_size: usize, args: &[usize], ret_locs: &'a [usize]) -> StackFrame<'a> {
        debug_assert!(!self.frames.is_empty());

        let last_frame = *(unsafe { self.frames.last().unwrap_unchecked() });
        let start_idx = last_frame.end_idx;
        let end_idx = start_idx + frame_size;

        self.values.resize(end_idx);

        let frame = StackFrame::new(start_idx, end_idx, ret_locs);
        let args_count = args.len();
        for i in 0..args_count {
            let arg = unsafe { *args.get_unchecked(i) };
            let value = last_frame.get_value(self, arg);
            frame.set_value(self, i, value);
        }
        self.frames.push(frame);

        frame
    }

    pub fn exit_frame(&mut self, rets: &[usize]) -> Option<StackFrame<'a>> {
        debug_assert!(!self.frames.is_empty());

        let prev_frame = unsafe { self.frames.pop().unwrap_unchecked() };
        if let Some(&current_frame) = self.frames.last() {
            let rets_count = rets.len();
            for i in 0..rets_count {
                let ret = unsafe { *rets.get_unchecked(i) };
                let ret_loc = unsafe { *prev_frame.ret_locs.get_unchecked(i) };
                let value = prev_frame.get_value(self, ret);
                current_frame.set_value(self, ret_loc, value);
            }
            Some(current_frame)
        } else {
            None
        }
    }
}
