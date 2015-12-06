use binding::array;
use types;

use super::traits::RawObject;

pub struct Array {
    value: types::rb_value
}

impl Array {
    pub fn new() -> Self {
        Array {
            value: array::new()
        }
    }

    pub fn push<T: RawObject>(&mut self, item: T) -> &mut Self {
        array::push(self.value(), item.value());

        self
    }
}

impl RawObject for Array {
    fn value(&self) -> types::rb_value {
        self.value
    }

    fn from_value(value: types::rb_value) -> Self {
        Array {
            value: value
        }
    }
}
