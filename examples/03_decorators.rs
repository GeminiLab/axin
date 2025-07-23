//! Decorators allow you to modify the behavior of functions by wrapping them with additional functionality. Unlike
//! decorators in some other languages, Axin decorators are called every time the function is executed, not just once.

use axin::axin;

fn timing_decorator<F>(func: F) -> i32
where
    F: FnOnce() -> i32,
{
    println!("⏱️  Starting timer...");
    let start = std::time::Instant::now();
    let result = func();
    let duration = start.elapsed();
    println!("⏱️  Function took: {:?}", duration);
    result
}

fn logging_decorator<F>(func: F) -> String
where
    F: FnOnce() -> String,
{
    println!("📋 Logging decorator: Before function");
    let result = func();
    println!("📋 Logging decorator: After function, result: {}", result);
    result
}

fn double_result<F>(func: F) -> i32
where
    F: FnOnce() -> i32,
{
    let result = func();
    println!("🔢 Doubling result from {} to {}", result, result * 2);
    result * 2
}

#[axin(decorator(timing_decorator))]
fn timed_calculation() -> i32 {
    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("Calculating important result...");
    42
}

#[axin(decorator(logging_decorator))]
fn logged_operation() -> String {
    println!("Performing string operation");
    "Hello, World!".to_string()
}

#[axin(decorator(double_result))]
fn doubled_math() -> i32 {
    println!("Computing base value");
    21
}

fn main() {
    println!("=== Decorator Functions Demo ===");

    println!("\n--- Timing decorator ---");
    let result1 = timed_calculation();
    println!("Final result: {}", result1);

    println!("\n--- Logging decorator ---");
    let result2 = logged_operation();
    println!("Final result: {}", result2);

    println!("\n--- Doubling decorator ---");
    let result3 = doubled_math();
    println!("Final result: {}", result3);
}
