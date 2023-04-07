use std::ptr::slice_from_raw_parts;
use crate::value::RtValue;

pub unsafe fn builtin_min(args: *mut RtValue, n_args: u32, rets: *mut RtValue) {
    assert!(n_args >= 2);
    let i32_data = &*slice_from_raw_parts::<i32>(args as *mut i32 as _, n_args as usize);
    (*rets).i = *i32_data.iter().min().unwrap_unchecked();
}

pub unsafe fn builtin_max(args: *mut RtValue, n_args: u32, rets: *mut RtValue) {
    assert!(n_args >= 2);
    let i32_data = &*slice_from_raw_parts::<i32>(args as *mut i32 as _, n_args as usize);
    (*rets).i = *i32_data.iter().max().unwrap_unchecked();
}

