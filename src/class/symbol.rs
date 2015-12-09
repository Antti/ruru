use std::convert::From;

use binding::util;
use types;

use super::traits::RawObject;

pub struct Symbol {
    value: types::rb_value
}

impl Symbol {
    pub fn new(string: &str) -> Self {
        Symbol {
            value: util::id_to_sym(util::internal_id(string))
        }
    }

    pub fn to_string(&self) -> String {
        util::id_to_name(util::sym_to_id(self.value()))
    }
}

impl From<types::rb_value> for Symbol {
    fn from(value: types::rb_value) -> Self {
        Symbol {
            value: value
        }
    }
}

impl RawObject for Symbol {
    fn value(&self) -> types::rb_value {
        self.value
    }
}
