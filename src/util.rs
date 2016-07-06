use std::ffi::{CStr, CString};

use binding::global::RubySpecialConsts;
use class::any_object::AnyObject;
use types::{c_char, c_int, InternalValue, Value};

use class::traits::Object;

pub fn cstr_as_string(str: *const c_char) -> String {
    unsafe { CStr::from_ptr(str).to_string_lossy().into_owned() }
}

pub fn str_to_cstring(str: &str) -> CString {
    CString::new(str).unwrap()
}

pub fn bool_to_value(state: bool) -> Value {
    let internal_value = match state {
        false => RubySpecialConsts::False,
        true => RubySpecialConsts::True,
    };

    Value::from(internal_value as InternalValue)
}

pub fn create_arguments(arguments: Vec<AnyObject>) -> (c_int, Vec<Value>) {
    (arguments.len() as c_int, arguments_to_values(arguments))
}

fn arguments_to_values(arguments: Vec<AnyObject>) -> Vec<Value> {
    arguments.iter()
        .map(|object| object.value())
        .collect::<Vec<Value>>()
}
