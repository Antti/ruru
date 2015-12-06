use types;
use unsafe_bindings::string;
use util;

pub fn string_new(string: &str) -> types::rb_value {
    unsafe {
        string::rb_str_new_cstr(util::str_as_ptr(string))
    }
}

pub fn string_from_value(value: types::rb_value) -> String {
    unsafe {
        let str = string::rb_string_value_cstr(&value);

        util::cstr_as_str(str)
    }
}
