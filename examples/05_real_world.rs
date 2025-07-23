//! Simulating a real-world scenario using Axin.

use axin::axin;

use std::time::Instant;

// Simulate logging system
fn log_api_start() {
    println!("🌐 API Request started");
}

fn log_api_end() {
    println!("🌐 API Request completed");
}

// Performance monitoring decorator
fn api_performance_monitor<F>(func: F) -> Result<String, String>
where
    F: FnOnce() -> Result<String, String>,
{
    let start = Instant::now();
    println!("📊 Performance: Monitoring API call");
    let result = func();
    let duration = start.elapsed();
    println!("📊 Performance: API call took {:?}", duration);
    result
}

// Error handling decorator
fn error_handler<F>(func: F) -> i32
where
    F: FnOnce() -> i32,
{
    println!("🛡️ Error Handler: Wrapping function");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(func));
    match result {
        Ok(value) => {
            println!("🛡️ Error Handler: Function completed successfully");
            value
        }
        Err(_) => {
            println!("🛡️ Error Handler: Caught panic, returning default value");
            -1
        }
    }
}

// Cache decorator
fn cache_decorator<F>(func: F) -> String
where
    F: FnOnce() -> String,
{
    println!("💾 Cache: Checking cache...");
    // Simulate cache lookup
    println!("💾 Cache: Cache miss, executing function");
    let result = func();
    println!("💾 Cache: Storing result in cache");
    result
}

// API endpoint simulation
#[axin(
    prologue(
        println!("🔧 Initializing request context");
        let request_id = "req_12345";
        println!("🔧 Request ID: {}", request_id);
    ),
    on_enter(log_api_start),
    decorator(api_performance_monitor),
    on_exit(log_api_end)
)]
fn get_user_profile() -> Result<String, String> {
    println!("👤 Fetching user profile from database");
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok("User: John Doe, Email: john@example.com".to_string())
}

// Data processing functionality
#[axin(
    prologue(println!("📊 Preparing data processing pipeline");),
    decorator(error_handler)
)]
fn process_data() -> i32 {
    println!("🔄 Processing large dataset");
    std::thread::sleep(std::time::Duration::from_millis(50));
    42
}

// Cached computation
#[axin(
    prologue(
        println!("🧮 Setting up calculation parameters");
        let precision = 0.001;
        println!("🧮 Using precision: {}", precision);
    ),
    decorator(cache_decorator)
)]
fn expensive_calculation() -> String {
    println!("💰 Performing expensive mathematical operation");
    std::thread::sleep(std::time::Duration::from_millis(200));
    "Result: 3.14159265359".to_string()
}

fn main() {
    println!("=== Real-world Usage Examples ===");

    println!("\n--- API Endpoint with full monitoring ---");
    match get_user_profile() {
        Ok(profile) => println!("✅ Profile: {}", profile),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n--- Data processing with error handling ---");
    let result = process_data();
    println!("📈 Processing result: {}", result);

    println!("\n--- Expensive calculation with caching ---");
    let calc_result = expensive_calculation();
    println!("🎯 Calculation result: {}", calc_result);
}
