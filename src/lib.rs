//! # Axin
//!
//! Axin is a powerful Rust procedural macro library that provides function instrumentation capabilities through the
//! `axin` attribute macro. This enables clean separation of concerns and reduces boilerplate code in applications that
//! require cross-cutting functionality like logging, timing, validation, or resource management.
//!
//! ## What can Axin do?
//!
//! Axin allows you change the behavior of functions in a declarative way using attributes. You can:
//! - add entry and exit hooks,
//! - insert statements at the beginning of function execution, and
//! - wrap functions with decorators.
//!
//! ### Entry and Exit Hooks
//!
//! These hooks allow you to execute custom functions when entering or exiting the target function. It's also possible
//! to specify arguments for these hooks, which can be used to pass context or configuration. Multiple hooks can be
//! specified both using separate declarations or within a single declaration.
//!
//! ```
//! use axin::axin;
//!
//! fn setup() {
//!     println!("Setting up");
//! }
//!
//! fn cleanup(msg: &str) {
//!     println!("Cleaning up: {}", msg);
//! }
//!
//! fn init_logging() {
//!     println!("Initializing logging");
//! }
//!
//! fn validate_env() {
//!     println!("Validating environment");
//! }
//!
//! // Single hooks
//! #[axin(on_enter(setup), on_exit(cleanup("Goodbye from function1!")))]
//! fn function1() {
//!     println!("Main logic");
//! }
//!
//! #[axin(on_enter(setup), on_enter(init_logging), on_exit(cleanup("Goodnight from function2!")))]
//! fn function2() {
//!     println!("Different logic");
//! }
//!
//! // Multiple hooks in single declaration
//! #[axin(on_enter(setup, init_logging, validate_env))]
//! fn function3() {
//!     println!("More logic");
//! }
//!
//! fn main() {
//!     function1();
//!     // Output:
//!     // Setting up
//!     // Main logic
//!     // Cleaning up: Goodbye from function1!
//!
//!     function2();
//!     // Output:
//!     // Setting up
//!     // Initializing logging
//!     // Different logic
//!     // Cleaning up: Goodnight from function2!
//!
//!     function3();
//!     // Setting up
//!     // Initializing logging
//!     // Validating environment
//!     // More logic
//! }
//! ```
//!
//! ### Prologue Statements
//!
//! Prologue statements allow you to insert arbitrary Rust code at the beginning of the function body. This can be very
//! useful sometimes, as the inserted code shares the same scope as the function, though hooks and decorators are better
//! choices for most use cases.
//!
//! ```
//! use axin::axin;
//!
//! #[axin(prologue(
//!     println!("Function starting");
//! ))]
//! fn my_function() {
//!     println!("Main logic");
//! }
//!
//! fn main() {
//!     my_function();
//!     // Output:
//!     // Function starting
//!     // Main logic
//! }
//! ```
//!
//! ### Decorators
//!
//! Decorators allow you to wrap the function with additional behavior. This is useful for cross-cutting concerns like
//! logging, monitoring, or authentication. Unlike decorators in some other languages, Axin decorators are called every
//! time the function is invoked, not just once at definition time.
//!
//! Decorators can have parameters, allowing you to customize their behavior based on the context in which they are
//! called. You can refer to the example below to see how to use decorators with and without parameters.
//!
//! ```
//! use axin::axin;
//!
//! fn timing_decorator<F, R>(func: F) -> R
//! where
//!    F: FnOnce() -> R,
//! {
//!    let start = std::time::Instant::now();
//!    println!("Starting timer...");
//!    let result = func();
//!    println!("Took: {:?}", start.elapsed());
//!    result
//! }
//!
//! #[axin(decorator(timing_decorator))]
//! fn expensive_computation() -> i32 {
//!     // Simulate work
//!     println!("Doing expensive computation...");
//!     std::thread::sleep(std::time::Duration::from_millis(100));
//!     42
//! }
//!
//! fn custom_logging_decorator<F, R>(msg: &'static str) -> impl FnOnce(F) -> R
//! where
//!     F: FnOnce() -> R,
//! {
//!     move |f| {
//!         println!("Custom log: {}", msg);
//!         f()
//!     }
//! }
//!
//! #[axin(decorator(custom_logging_decorator("Hello from decorated function!")))]
//! fn decorated_function() -> String {
//!     println!("Doing something important...");
//!     "Decorated result".to_string()
//! }
//!
//! fn main() {
//!     let result = expensive_computation();
//!     println!("Result: {}", result);
//!     // Output:
//!     // Starting timer...
//!     // Doing expensive computation...
//!     // Took: ...
//!     // Result: 42
//!
//!     let decorated_result = decorated_function();
//!     println!("Decorated result: {}", decorated_result);
//!     // Output:
//!     // Custom log: Hello from decorated function!
//!     // Doing something important...
//!     // Decorated result: Decorated result
//! }
//! ```
//!
//! Decorators do not support variadic arguments, due to the limitation of Rust.
//!
//! ## Order of Execution
//!
//! The order of execution for the various Axin features is as follows:
//! 1. Entry hook functions (if specified) are executed first in declaration order, then
//! 2. Decorator function (if specified) is called, and when it calls the original function,
//! 3. Prologue statements (if specified) are executed, and then
//! 4. The original function body is executed, after which
//! 5. The control flow returns to the decorator, and after it completes,
//! 6. The exit hook functions (if specified) are executed last in declaration order.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod args;
mod generator;

use args::AxinArgs;
use generator::{generate_enhanced_function, process_attribute_args};

/// An attribute procedural macro that enhances functions with entry and exit hooks, decorators, and prologue statements.
///
/// For more details, see the [Axin documentation](crate).
///
/// ## Example
///
/// ```
/// use axin::axin;
///
/// fn setup() {
///     println!("Starting function");
/// }
///
/// fn cleanup() {
///     println!("Function completed");
/// }
///
/// fn timing_decorator<F, R>(func: F) -> R
/// where F: FnOnce() -> R
/// {
///     let start = std::time::Instant::now();
///     let result = func();
///     println!("Execution time: {:?}", start.elapsed());
///     result
/// }
///
/// #[axin(
///     prologue(println!("Initializing");),
///     on_enter(setup),
///     decorator(timing_decorator),
///     on_exit(cleanup)
/// )]
/// fn instrumented_function() -> i32 {
///     println!("Core logic");
///     42
/// }
///
/// fn main() {
///     let result = instrumented_function();
///     println!("Result: {}", result);
///
///     // Output:
///     // Starting function
///     // Initializing
///     // Core logic
///     // Execution time: ...
///     // Function completed
///     // Result: 42
/// }
/// ```
#[proc_macro_attribute]
pub fn axin(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Parse attribute parameters
    if !args.is_empty() {
        let attribute_args = match syn::parse::<AxinArgs>(args) {
            Ok(args) => args,
            Err(e) => return e.to_compile_error().into(),
        };

        let (prologue_stmts, decorator_fn, on_enter_funcs, on_exit_funcs) =
            process_attribute_args(attribute_args);

        // Process function enhancement according to the new design
        generate_enhanced_function(
            input_fn,
            prologue_stmts,
            decorator_fn,
            on_enter_funcs,
            on_exit_funcs,
        )
        .into()
    } else {
        quote! {
            #input_fn
        }
        .into()
    }
}
