use types::{Argc, c_char, c_int, CallbackPtr, Value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_class_new_instance(argc: Argc, argv: *const Value, klass: Value) -> Value;
    pub fn rb_define_class(name: *const c_char, superclass: Value) -> Value;
    pub fn rb_define_method(klass: Value, name: *const c_char, callback: CallbackPtr, argc: Argc);
    pub fn rb_define_module(name: *const c_char) -> Value;
    pub fn rb_obj_class(object: Value) -> Value;

    pub fn rb_define_singleton_method(klass: Value,
                                      name: *const c_char,
                                      callback: CallbackPtr,
                                      argc: Argc);
}
