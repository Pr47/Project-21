use crate::r25_300::value::RtValue;

pub type RawFunction = fn(args: *mut RtValue, n_args: u32, rets: *mut RtValue);
