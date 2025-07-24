//! Integration tests for the `axin` macro
//!
//! This test module is "self-explaining", it demonstrates how to use the `axin` macro with various parameters, and it
//! also shows that how to use the macro in a test context.

/// Utilities for tests
#[macro_use]
mod utils {
    use std::sync::Mutex;

    /// A static variable to capture the output of the test functions, the lock here is just for interior mutability.
    static OUTPUT: Mutex<String> = Mutex::new(String::new());
    /// A static lock to ensure single-threaded access to the test functions.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    pub fn output(msg: impl Into<String>) {
        let mut output = OUTPUT.lock().unwrap();
        output.push_str(&msg.into());
    }

    macro_rules! println_test {
        ($($arg:tt)*) => {
            $crate::utils::output(format!($($arg)*));
            $crate::utils::output("\n");
        }
    }

    /// Decorator for testcases to ensure single-threaded execution and capture output.
    pub fn single_threaded_test<F, R, O>(expected_output: O) -> impl FnOnce(F) -> R
    where
        F: FnOnce() -> R,
        O: AsRef<str>,
    {
        move |f: F| {
            let _lock = TEST_LOCK.lock().unwrap(); // Ensure single-threaded access
            OUTPUT.lock().unwrap().clear(); // Clear previous output

            let result = f();

            let expected_output = expected_output.as_ref();
            let actual_output = OUTPUT.lock().unwrap();
            assert_eq!(
                actual_output.as_str(),
                expected_output,
                "Test output did not match expected:\nExpected: {}\nActual: {}",
                expected_output,
                actual_output
            );

            result
        }
    }
}

/// Hooks and decorators for the tests
mod testee {
    use std::fmt;

    pub fn on_enter_hook() {
        println_test!("Entering hook");
    }

    pub fn on_exit_hook() {
        println_test!("Exiting hook");
    }

    pub fn parameterized_hook(param: &str) {
        println_test!("Param hook: {}", param);
    }

    pub fn simple_decorator<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        println_test!("Entering decorator");
        let result = f();
        println_test!("Exiting decorator");
        result
    }

    pub fn simple_decorator_with_param<F, P, R>(f: F, param: P) -> R
    where
        F: FnOnce(P) -> R,
        P: fmt::Display + Copy,
    {
        println_test!("Entering decorator: {}", param);
        let result = f(param);
        println_test!("Exiting decorator: {}", param);
        result
    }

    pub fn parameterized_decorator<F, P, R>(param: P) -> impl FnOnce(F) -> R
    where
        F: FnOnce() -> R,
        P: fmt::Display + Copy,
    {
        move |f: F| {
            println_test!("Entering param decorator: {}", param);
            let result = f();
            println_test!("Exiting param decorator: {}", param);
            result
        }
    }

    pub fn parameterized_decorator_with_param<F, P, Q, R>(param: P) -> impl FnOnce(F, Q) -> R
    where
        F: FnOnce(Q) -> R,
        P: fmt::Display + Copy,
        Q: fmt::Display + Copy,
    {
        move |f: F, arg: Q| {
            println_test!("Entering param decorator: {}", param);
            println_test!("User arg: {}", arg);
            let result = f(arg);
            println_test!("Exiting param decorator: {}", param);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use axin::axin;

    use super::{testee::*, utils::*};

    // each test here contains two functions:
    // 1. The function to be `axin`-ed, and tested.
    // 2. The test function that calls the first one with the decorator to capture the output and check it.

    // test simple hooks
    #[axin(on_enter(on_enter_hook), on_exit(on_exit_hook))]
    fn test_hooks() {
        println_test!("Inside test_hooks function");
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        "Entering hook\nInside test_hooks function\nExiting hook\n"
    )))]
    fn call_test_hooks() {
        test_hooks();
    }

    // test parameterized hooks
    #[axin(on_enter(parameterized_hook("test_param")), on_exit(on_exit_hook))]
    fn test_parameterized_hooks() {
        println_test!("Inside test_parameterized_hooks function");
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        "Param hook: test_param\nInside test_parameterized_hooks function\nExiting hook\n"
    )))]
    fn call_test_parameterized_hooks() {
        test_parameterized_hooks();
    }

    // test prologue functionality
    #[axin(prologue(println_test!("Prologue statement executed")))]
    fn test_prologue() {
        println_test!("Inside test_prologue function");
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        "Prologue statement executed\nInside test_prologue function\n"
    )))]
    fn call_test_prologue() {
        test_prologue();
    }

    // test multiple prologue statements with hooks
    #[axin(prologue(
        println_test!("Prologue step 1 executed");
        println_test!("Prologue step 2 executed");
    ), on_enter(on_enter_hook), on_exit(on_exit_hook))]
    fn test_multiple_prologue() {
        println_test!("Inside test_multiple_prologue function");
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        r#"Entering hook
Prologue step 1 executed
Prologue step 2 executed
Inside test_multiple_prologue function
Exiting hook
"#
    )))]
    fn call_test_multiple_prologue() {
        test_multiple_prologue();
    }

    // test simple decorator
    #[axin(decorator(simple_decorator))]
    fn test_simple_decorator() {
        println_test!("Inside test_simple_decorator function");
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        "Entering decorator\nInside test_simple_decorator function\nExiting decorator\n"
    )))]
    fn call_test_simple_decorator() {
        test_simple_decorator();
    }

    // test simple decorator with parameters
    #[axin(decorator(simple_decorator_with_param))]
    fn test_simple_decorator_with_param(i: i32) -> i32 {
        println_test!("Inside test_simple_decorator_with_param function: {}", i);
        i + 1
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        "Entering decorator: 42\nInside test_simple_decorator_with_param function: 42\nExiting decorator: 42\n"
    )))]
    fn call_test_simple_decorator_with_param() {
        let result = test_simple_decorator_with_param(42);
        assert_eq!(result, 43, "Expected result to be 43");
    }

    // test parameterized decorator
    #[axin(decorator(parameterized_decorator("test_param")))]
    fn test_parameterized_decorator() {
        println_test!("Inside test_parameterized_decorator function");
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        r#"Entering param decorator: test_param
Inside test_parameterized_decorator function
Exiting param decorator: test_param
"#
    )))]
    fn call_test_parameterized_decorator() {
        test_parameterized_decorator();
    }

    // test parameterized decorator with parameters
    #[axin(decorator(parameterized_decorator_with_param("test_param")))]
    fn test_parameterized_decorator_with_param(i: i32) -> i32 {
        println_test!(
            "Inside test_parameterized_decorator_with_param function: {}",
            i
        );
        i + 1
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        r#"Entering param decorator: test_param
User arg: 100
Inside test_parameterized_decorator_with_param function: 100
Exiting param decorator: test_param
"#
    )))]
    fn call_test_parameterized_decorator_with_param() {
        let result = test_parameterized_decorator_with_param(100);
        assert_eq!(result, 101, "Expected result to be 101");
    }

    // test mixed multiple hooks with other features
    #[axin(
        on_enter(on_enter_hook, parameterized_hook("second_enter_hook")),
        prologue(println_test!("Prologue with multiple hooks");),
        on_exit(parameterized_hook("first_exit_hook")),
        on_exit(on_exit_hook)
    )]
    fn test_mixed_multiple_hooks() {
        println_test!("Inside test_mixed_multiple_hooks function");
    }

    #[test]
    #[axin(decorator(single_threaded_test(
        r#"Entering hook
Param hook: second_enter_hook
Prologue with multiple hooks
Inside test_mixed_multiple_hooks function
Param hook: first_exit_hook
Exiting hook
"#
    )))]
    fn call_test_mixed_multiple_hooks() {
        test_mixed_multiple_hooks();
    }
}
