extern crate libc;

pub use libc::{c_char, c_int, c_long};

#[allow(non_camel_case_types)]
pub type rb_value = libc::uintptr_t;
