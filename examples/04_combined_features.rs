//! Prologues, entry/exit hooks, and decorators can be combined to enhance function behavior in Rust. As far, only one
//! can be specified for each group.

use axin::axin;

fn initialize() {
    println!("🔧 System initialization");
}

fn finalize() {
    println!("🏁 System finalization");
}

fn performance_monitor<F>(func: F) -> i32
where
    F: FnOnce() -> i32,
{
    println!("📊 Performance monitor: Starting");
    let start = std::time::Instant::now();
    let result = func();
    let duration = start.elapsed();
    println!("📊 Performance monitor: Completed in {:?}", duration);
    result
}

fn security_wrapper<F>(func: F) -> String
where
    F: FnOnce() -> String,
{
    println!("🔒 Security check: Validating permissions");
    let result = func();
    println!("🔒 Security check: Operation authorized");
    result
}

#[axin(
    prologue(
        println!("📝 Setting up variables");
        println!("📝 Configuring environment");
    ),
    on_enter(initialize),
    decorator(performance_monitor),
    on_exit(finalize)
)]
fn complex_operation() -> i32 {
    println!("💼 Executing core business logic");
    std::thread::sleep(std::time::Duration::from_millis(50));
    println!("💼 Processing data");
    100
}

#[axin(
    prologue(println!("🔐 Preparing secure context");),
    on_enter(initialize),
    decorator(security_wrapper),
    on_exit(finalize)
)]
fn secure_operation() -> String {
    println!("🔑 Handling sensitive data");
    "Classified Information".to_string()
}

fn step2_begin() {
    println!("2️⃣ OnEnter: Begin");
}

fn step6_end() {
    println!("6️⃣ OnExit: End");
}

fn order_decorator<F>(f: F) -> i32
where
    F: FnOnce() -> i32,
{
    println!("3️⃣ Decorator: Before");
    let result = f();
    println!("5️⃣ Decorator: After");
    result
}

// Demonstrate execution order
#[axin(
    prologue(println!("1️⃣ Prologue: Setup");),
    on_enter(step2_begin),
    decorator(order_decorator),
    on_exit(step6_end)
)]
fn execution_order_demo() -> i32 {
    println!("4️⃣ Function Body: Main work");
    42
}

fn main() {
    println!("=== Combined Features Demo ===");

    println!("\n--- Complex operation with all features ---");
    let result1 = complex_operation();
    println!("Final result: {}", result1);

    println!("\n--- Secure operation ---");
    let result2 = secure_operation();
    println!("Final result: {}", result2);

    println!("\n--- Execution order demonstration ---");
    let result3 = execution_order_demo();
    println!("Final result: {}", result3);
}
