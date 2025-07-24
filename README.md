# Axin

[![Crates.io](https://img.shields.io/crates/v/axin.svg)](https://crates.io/crates/axin)
[![Documentation](https://docs.rs/axin/badge.svg)](https://docs.rs/axin)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust procedural macro library for function instrumentation. Axin provides the attribute procedural macro `axin` that enable clean separation of cross-cutting concerns such as logging, timing, validation, and resource management.

## Features

- **Entry & Exit Hooks**: Execute functions before and after target function execution
- **Decorators**: Wrap functions with additional behavior using the decorator pattern
- **Prologue**: Insert statements directly at function entry
- **Composable**: Combine multiple features seamlessly
- **Zero Runtime Cost**: All transformations occur at compile time

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
axin = "0.1.0"
```

## Usage

### Basic Example

```rust
use axin::axin;

fn setup() {
    println!("Function starting");
}

fn log_start(message: &str) {
    println!("Starting: {}", message);
}

fn cleanup() {
    println!("Function completed");
}

#[axin(on_enter(setup, log_start("my_function")), on_exit(cleanup))]
fn my_function() {
    println!("Executing main logic");
}

fn main() {
    my_function();
    // Output:
    // Function starting
    // Starting: my_function
    // Executing main logic  
    // Function completed
}
```

### Decorators

```rust
use axin::axin;

fn timing_decorator<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = func();
    println!("Execution time: {:?}", start.elapsed());
    result
}

#[axin(decorator(timing_decorator))]
fn measured_function() -> i32 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}
```

### Prologue Statements

```rust
use axin::axin;

#[axin(prologue(
    println!("Initializing context");
    let start_time = std::time::Instant::now();
))]
fn function_with_setup() {
    println!("Main logic executed");
    // start_time variable is available in this scope
}
```

### Combined Usage

```rust
use axin::axin;

fn init() { println!("Initializing"); }
fn cleanup() { println!("Cleaning up"); }

fn monitor<F, R>(f: F) -> R where F: FnOnce() -> R {
    println!("Monitoring started");
    let result = f();
    println!("Monitoring completed");
    result
}

#[axin(
    prologue(println!("Setting up context");),
    on_enter(init),
    decorator(monitor),
    on_exit(cleanup)
)]
fn complex_function() -> String {
    println!("Core logic");
    "result".to_string()
}
```

## Execution Order

When combining features, execution follows this order:

- Entry hooks (in declaration order)
- Decorator
- Prologue statements
- Original function body
- Exit hooks (in declaration order)

## API Reference

### Parameters Syntax

```rust
#[axin(
    prologue(statement1; statement2; ...),
    on_enter(function_name),        // Single function
    on_enter(func1, func2, func3),  // Multiple functions in one declaration
    on_enter(func4),                // Additional separate declaration
    decorator(decorator_function),  // or decorator(parameterized_decorator(param1, param2)),
    on_exit(cleanup_function),      // Single function
    on_exit(cleanup1, cleanup2),    // Multiple functions in one declaration
)]
```

- `prologue(statements...)` - Insert statements at function start
- `on_enter(function)` - Execute function(s) before main function
  - `on_enter(function_with_args("arg1", "arg2"))` - Pass arguments to the entry function
  - `on_enter(func1, func2, func3)` - Execute multiple functions in sequence
  - Multiple `on_enter` declarations can be used and will execute in order
- `on_exit(function)` - Execute function(s) after main function
  - `on_exit(function_with_args("arg1", "arg2"))` - Pass arguments to the exit function
  - `on_exit(func1, func2, func3)` - Execute multiple functions in sequence
  - Multiple `on_exit` declarations can be used and will execute in order
- `decorator(function)` - Wrap function with decorator
  - `decorator(function_with_args("arg1", "arg2"))` - Pass arguments to the decorator

All parameters are optional and can be combined in any order.

## Examples

See the `examples/` directory for comprehensive usage examples:

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
