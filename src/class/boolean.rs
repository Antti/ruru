use std::convert::From;

use types::Value;
use util::bool_to_value;

use super::traits::Object;

/// `TrueClass` and `FalseClass`
pub struct Boolean {
    value: Value,
}

impl Boolean {
    /// Creates a new instance boolean value from `bool`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Boolean, VM};
    /// # VM::init();
    ///
    /// assert_eq!(Boolean::new(true).to_bool(), true);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// true == true
    /// ```
    pub fn new(state: bool) -> Self {
        Boolean { value: bool_to_value(state) }
    }

    /// Retrieves a `bool` value from `Boolean`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Boolean, VM};
    /// # VM::init();
    ///
    /// assert_eq!(Boolean::new(true).to_bool(), true);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// true == true
    /// ```
    pub fn to_bool(&self) -> bool {
        self.value().is_true()
    }
}

impl From<Value> for Boolean {
    fn from(value: Value) -> Self {
        Boolean { value: value }
    }
}

impl Object for Boolean {
    fn value(&self) -> Value {
        self.value
    }
}
