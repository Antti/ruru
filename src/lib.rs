extern crate libc;

mod binding;
mod class;
mod types;
mod unsafe_binding;
mod util;

pub use class::array::Array;
pub use class::boolean::Boolean;
pub use class::class::Class;
pub use class::hash::Hash;
pub use class::object::Object;
pub use class::string::RString;
pub use class::symbol::Symbol;
pub use class::vm::VM;

pub use class::traits;

#[test]
fn it_works() {
}
