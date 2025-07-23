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

fn cleanup() {
    println!("Function completed");
}

#[axin(on_enter(setup), on_exit(cleanup))]
fn my_function() {
    println!("Executing main logic");
}

fn main() {
    my_function();
    // Output:
    // Function starting
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

- Entry hook
- Decorator
- Prologue statements
- Original function body
- Exit hook

## API Reference

### Parameters Syntax

```rust
#[axin(
    prologue(statement1; statement2; ...),
    on_enter(function_name),        // or on_enter(function_with_args("arg1", "arg2")),
    decorator(decorator_function),  // or decorator(parameterized_decorator(param1, param2)),
    on_exit(cleanup_function)       // or on_exit(cleanup_function_with_args("arg1", "arg2")),
)]
```

- `prologue(statements...)` - Insert statements at function start
- `on_enter(function)` - Execute function before main function
  - `on_enter(function_with_args("arg1", "arg2"))` - Pass arguments to the entry function
- `on_exit(function)` - Execute function after main function
  - `on_exit(function_with_args("arg1", "arg2"))` - Pass arguments to the exit function
- `decorator(function)` - Wrap function with decorator
  - `decorator(function_with_args("arg1", "arg2"))` - Pass arguments to the decorator

All parameters are optional and can be combined in any order.

## Examples

See the `examples/` directory for comprehensive usage examples:

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
