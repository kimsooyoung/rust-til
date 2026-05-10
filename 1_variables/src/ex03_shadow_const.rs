//! Topic 3 — Shadowing (`let` again with the same name) and compile-time `const` values.
//!
//! Run via: `cargo run --bin ex03_shadow_const`

use crate::helpers::section;

/// Shows how shadowing creates a *new* binding (and may change type), plus simple `const`s.
pub fn run() {
    section("Variable shadowing (new binding, can change type)");
    let spaces = "   ";
    println!("shadow 1 (str): len = {}", spaces.len());
    let spaces = spaces.len();
    println!("shadow 2 (usize): {spaces}");

    let raw = "100";
    let raw: i32 = raw.parse().expect("digits");
    println!("parsed shadowed `raw` as i32: {raw}");

    section("Constants (compile-time, UPPER_SNAKE_CASE, type required)");
    const MAX_POINTS: u32 = 100_000;
    const DEFAULT_TAX_RATE_PERCENT: f64 = 8.25;
    println!("MAX_POINTS = {MAX_POINTS}, DEFAULT_TAX_RATE_PERCENT = {DEFAULT_TAX_RATE_PERCENT}");
    // `const` values must be evaluable at compile time; they are inlined where used.
}
