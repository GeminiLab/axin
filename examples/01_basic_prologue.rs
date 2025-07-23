//! Prologue allows inserting statements at the beginning of the function body.

use axin::axin;

#[axin(prologue(println!("Initializing function");))]
fn simple_prologue() {
    println!("Function body executing");
}

#[axin(prologue(
    println!("Step 1: Setup");
    println!("Step 2: Initialize");
    println!("Step 3: Ready");
))]
fn multi_step_prologue() {
    println!("Main function logic");
}

fn main() {
    println!("=== Basic Prologue Demo ===");

    println!("\n--- Simple prologue ---");
    simple_prologue();

    println!("\n--- Multi-step prologue ---");
    multi_step_prologue();
}
