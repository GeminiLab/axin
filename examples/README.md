# Axin Examples

This directory contains comprehensive examples demonstrating the capabilities of the `axin` crate.

## Examples Overview

### Basic Examples

#### `01_basic_prologue.rs` - Prologue Functionality
Demonstrates inserting statements at function entry:
- Single prologue statement
- Multiple prologue statements

```bash
cargo run --example 01_basic_prologue
```

#### `02_entry_exit.rs` - Entry and Exit Hooks
Shows `on_enter` and `on_exit` functionality:
- Function setup before execution
- Function cleanup after execution
- Using both hooks together
- Entry and exit hooks with arguments

```bash
cargo run --example 02_entry_exit
```

#### `03_decorators.rs` - Decorator Patterns
Demonstrates function wrapping with decorators:
- Timing decorator for performance monitoring
- Logging decorator for function tracing
- Result transformation decorator

```bash
cargo run --example 03_decorators
```

### Advanced Examples

#### `04_combined_features.rs` - Feature Composition
Shows how to combine all features:
- Complete feature integration
- Execution order demonstration
- Different combination patterns
- Limitations: only one of each feature type per function

```bash
cargo run --example 04_combined_features
```

#### `05_real_world.rs` - Real-world Applications
Practical usage scenarios simulating real applications:
- API endpoint monitoring with performance tracking
- Error handling patterns with panic recovery
- Caching mechanisms for expensive computations

```bash
cargo run --example 05_real_world
```

## Feature Documentation

### Prologue
Insert code at the beginning of the function body:
```rust
#[axin(prologue(
    println!("Setup code");
    let var = 42;
))]
fn my_function() {
    // Function body has access to var
}
```

### Entry/Exit Hooks
Call specified functions before and after function execution:
```rust
#[axin(on_enter(setup_function), on_exit(cleanup_function))]
fn my_function() {
    // Function body
}

// Hooks can also accept arguments
#[axin(on_enter(log_start("function_name")), on_exit(log_end("function_name")))]
fn logged_function() {
    // Function body
}
```

### Decorators
Wrap functions with additional behavior:
```rust
fn timing_decorator<F>(func: F) -> i32
where F: FnOnce() -> i32
{
    let start = std::time::Instant::now();
    let result = func();
    println!("Execution time: {:?}", start.elapsed());
    result
}

#[axin(decorator(timing_decorator))]
fn timed_function() -> i32 {
    42
}
```

## Execution Order

When features are combined, execution follows this sequence:
1. `on_enter()` - Entry function
2. `decorator()` - Decorator function
3. `prologue` - Prologue statements
4. Original function body (wrapped by decorator if present)
5. `on_exit()` - Exit function

This order is demonstrated in the execution_order_demo function in `04_combined_features.rs`.
