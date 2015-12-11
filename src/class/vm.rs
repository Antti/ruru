use std::slice;

use binding::vm;
use class::object;
use types;

pub struct VM;

impl VM {
    pub fn init() {
        vm::init();
    }

    pub fn parse_arguments(argc: types::argc, arguments: *const types::rb_value) -> Vec<object::Object> {
        let arguments = unsafe {
            slice::from_raw_parts(arguments, argc as usize).to_vec()
        };

        arguments.iter().map(|value| object::Object::from(*value)).collect()
    }
}
