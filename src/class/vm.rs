use std::slice;

use binding::vm;
use types::{Argc, Value};

use {AnyObject, Proc};

/// Virtual Machine and helpers
pub struct VM;

impl VM {
    /// Initializes Ruby virtual machine.
    ///
    /// This function should **ONLY** be used if you write a standalone application which calls
    /// Ruby itself, for example:
    ///
    /// - Sidekiq-like background processing
    ///
    /// - Unicorn-like web server
    ///
    /// In these cases it should be called before any interaction with Ruby.
    ///
    /// If you write a library which is being connected to Ruby in runtime (e.g. some gem), this
    /// function should not be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, VM};
    ///
    /// VM::init();
    ///
    /// // VM started, able to use Ruby now
    /// // ...
    ///
    /// Class::new("SomeClass", None); // etc
    /// ```
    pub fn init() {
        vm::init();
    }

    /// Requires Ruby source file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::VM;
    /// # VM::init();
    ///
    /// VM::require("some_ruby_file");
    /// ```
    pub fn require(name: &str) {
        vm::require(name);
    }

    /// Converts a block given to current method to a `Proc`
    ///
    /// It works similarly to `def method(&block)` which converts block to `Proc`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use]
    /// extern crate ruru;
    ///
    /// use ruru::{Class, Object, Proc, RString, VM};
    ///
    /// class!(Greeter);
    ///
    /// methods!(
    ///     Greeter,
    ///     itself,
    ///
    ///     fn greet_rust_with() -> RString {
    ///         let greeting_template = VM::block_proc();
    ///         let name = RString::new("Rust").to_any_object();
    ///
    ///         greeting_template.call(vec![name]).try_convert_to::<RString>().unwrap()
    ///     }
    /// );
    ///
    /// fn main() {
    ///     Class::new("Greeter", None).define(|itself| {
    ///         itself.def_self("greet_rust_with", greet_rust_with);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Greeter
    ///   def self.greet_rust_with(greeting_template)
    ///     greeting_template.call('Rust')
    ///   end
    /// end
    ///
    /// Greeter.greet_rust_with do |name|
    ///   "Hello, #{name}!"
    /// end
    /// # => "Hello, Rust!"
    /// ```
    pub fn block_proc() -> Proc {
        Proc::from(vm::block_proc())
    }

    // TODO: Move to other struct
    /// Converts a pointer `AnyObject` array to `Vec<AnyObject>`.
    ///
    /// This function is a helper for callbacks.
    ///
    /// Later it will be moved to other struct, because it is not related to VM itself.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Boolean, Class, Object, RString, VM};
    ///
    /// #[no_mangle]
    /// pub extern fn string_eq(argc: Argc, argv: *const AnyObject, itself: RString) -> Boolean {
    ///     let argv = VM::parse_arguments(argc, argv);
    ///     let other_string = argv[0].try_convert_to::<RString>().unwrap();
    ///
    ///     Boolean::new(itself.to_string() == other_string.to_string())
    /// }
    ///
    /// fn main() {
    ///     Class::from_existing("String").define_method("==", string_eq);
    /// }
    /// ```
    pub fn parse_arguments(argc: Argc, arguments: *const AnyObject) -> Vec<AnyObject> {
        unsafe { slice::from_raw_parts(arguments, argc as usize).to_vec() }
    }

    pub fn thread_call_without_gvl<F, R, G>(func: F, unblock_func: Option<G>) -> R
        where F: FnMut() -> R,
              G: FnMut()
    {
        vm::thread_call_without_gvl(func, unblock_func)
    }

    pub fn thread_call_without_gvl2<F, R, G>(func: F, unblock_func: Option<G>) -> R
        where F: FnMut() -> R,
              G: FnMut()
    {
        vm::thread_call_without_gvl2(func, unblock_func)
    }

    pub fn thread_call_with_gvl<F, R>(func: F) -> R
        where F: FnMut() -> R
    {
        vm::thread_call_with_gvl(func)
    }

    pub fn protect<F>(func: F) -> Result<Value, i32>
        where F: FnMut()
    {
        vm::protect(func)
    }
}
