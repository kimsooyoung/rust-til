//! Topic 1 — Immutable vs `mut` bindings, and updating a value without changing its type.
//!
//! Run via: `cargo run --bin ex01_bindings`

use crate::helpers::section;

/// Demonstrates default immutability, `mut`, and reassignment within the same type.
pub fn run() {
    section("Immutable vs mutable bindings");
    let answer = 42;
    println!("answer (immutable): {answer}");
    // `answer = 0;` would not compile: cannot assign twice to an immutable binding.

    let mut score = 0;
    println!("score start: {score}");
    score += 10;
    println!("after += 10: {score}");
    score = 100;
    println!("score after updates: {score}");

    section("Changing the value (same type only)");
    let mut count: u32 = 1;
    println!("count start: {count}");
    count = 2;
    count += 5;
    println!("count: {count}");
    // Changing the *type* of `count` in place is not allowed; use shadowing (`let count = …`)
    // or a new variable name instead.
}
