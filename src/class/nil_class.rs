use std::convert::From;

use binding::global::RubySpecialConsts;
use types::{InternalValue, Value, ValueType};

use traits::{Object, VerifiedObject};

/// `NilClass`
#[derive(Debug, PartialEq)]
pub struct NilClass {
    value: Value,
}

impl NilClass {
    /// Creates a new instance of `NilClass` (`nil`).
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{NilClass, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// assert!(NilClass::new().value().is_nil());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// nil.nil? == true
    /// ```
    pub fn new() -> Self {
        NilClass { value: Value::from(RubySpecialConsts::Nil as InternalValue) }
    }
}

impl From<Value> for NilClass {
    fn from(value: Value) -> Self {
        NilClass { value: value }
    }
}

impl Object for NilClass {
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for NilClass {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.value().ty() == ValueType::Nil
    }

    fn error_message() -> String {
        "Error converting to NilClass".to_string()
    }
}
