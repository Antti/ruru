use bindings::globals;
use bindings::class;
use types;

use super::traits;

pub struct Class {
    value: types::rb_value
}

impl traits::RawObject for Class {
    fn value(&self) -> types::rb_value {
        self.value
    }

    fn from_value(value: types::rb_value) -> Self {
        Class {
            value: value
        }
    }
}

impl Class {
    // TODO: replace rb_cObject with optional superclass
    pub fn new(name: &str) -> Self {
        Class {
            value: class::define_class(name, globals::rb_cObject)
        }
    }

    pub fn define_method(&self,
                         name: &str,
                         callback: extern fn(types::rb_value) -> types::rb_value,
                         argc: i32) {
        class::define_method(self.value, name, callback, 0);
    }

    pub fn def_with_closure(&self,
                         name: &str,
                         callback: extern fn(types::rb_value) -> types::rb_value,
                         argc: i32) {
        class::define_method(self.value, name, callback, 0);
    }
}
