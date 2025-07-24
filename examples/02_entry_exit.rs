//! Entry and exit hooks allow you to execute custom functions when entering or exiting a target function.

use axin::axin;

fn setup() {
    println!("🚀 Function is starting...");
}

fn cleanup() {
    println!("✅ Function completed!");
}

fn log_start(msg: impl AsRef<str>) {
    println!("📝 Logging function entry: {}", msg.as_ref());
}

fn log_end(msg: impl AsRef<str>) {
    println!("📝 Logging function exit: {}", msg.as_ref());
}

#[axin(on_enter(setup))]
fn with_setup() {
    println!("Doing some work");
}

#[axin(on_exit(cleanup))]
fn with_cleanup() {
    println!("Doing some work");
}

#[axin(
    on_enter(setup, log_start("with_both")),
    on_exit(log_end("with_both")),
    on_exit(cleanup)
)]
fn with_both() {
    println!("Doing some work with full logging");
}

fn main() {
    println!("=== Entry/Exit Functions Demo ===");

    println!("\n--- Function with setup ---");
    with_setup();

    println!("\n--- Function with cleanup ---");
    with_cleanup();

    println!("\n--- Function with both and hooks with args ---");
    with_both();
}
