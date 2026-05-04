//! Chapter 3 — Functions basics
//!
//! Topics: tuple parameters and return values, block expressions (value of a block is its
//! last expression), closures, reading a line from standard input.

// -----------------------------------------------------------------------------
// Function taking a tuple argument
// -----------------------------------------------------------------------------
// `(i32, f32, bool)` is one compound type. Every element here is `Copy`, so the whole tuple is
// copied into the callee (not moved). If the tuple held something like `String`, move rules apply.
fn print_second_value(tup: (i32, f32, bool)) -> f32 {
    println!("second value: {}", tup.1); // `.0` first field, `.1` second, …
    tup.1
}

// -----------------------------------------------------------------------------
// Returning “multiple values” with a tuple
// -----------------------------------------------------------------------------
// Rust has no syntax for multiple return values; use a tuple (or a struct) instead.
// At the call site, **destructuring** with `let (a, b) = ...` unpacks the tuple cleanly.
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    // Integer `/` and `%` panic when `divisor` is zero; keep this example safe.
    assert_ne!(divisor, 0, "divisor must not be zero");

    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

fn main() {
    // -------------------------------------------------------------------------
    // Tuple demo: field access and passing into a function
    // -------------------------------------------------------------------------
    let my_tuple = (1, 2.0, true);
    let second = print_second_value(my_tuple);
    println!("returned second field: {second}");

    // -------------------------------------------------------------------------
    // Block expression: the **last expression** in `{ ... }` (no trailing `;`) is the block’s value.
    // -------------------------------------------------------------------------
    // The block below evaluates to `70000`, which becomes the value of `usd_to_krw`.
    // `let usd = 50` lives only inside the block (separate scope from the outer function).
    let usd_to_krw = {
        let usd = 50;
        usd * 1400 // No `;` here — otherwise the block would be `()` instead of this number.
    };
    println!("USD 50 dollar is KRW {}", usd_to_krw);

    // -------------------------------------------------------------------------
    // Tuple return + destructuring
    // -------------------------------------------------------------------------
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} remainder {}", quotient, remainder);

    // -------------------------------------------------------------------------
    // Closures: `|args| expr` or `|| { ... }` — anonymous functions you can store in a variable
    // -------------------------------------------------------------------------
    // `|name: &str|` — one argument. `format!` builds and returns a `String`.
    let greeting = |name: &str| format!("Hello, {}!", name);
    println!("greeting: {}", greeting("Sooyoung"));

    // Zero-argument closure. You may annotate the return type: `|| -> u32 { ... }`.
    // When you `cargo run` in a terminal, this prompts you to type your age on stdin.
    let ask_for_age = || -> u32 {
        use std::io::{self, Write};

        print!("Enter your age: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // `trim()` strips the newline, then parse to `u32`. Panics on bad input (fine for learning).
        // In production code, handle the `Result` from `parse::<u32>()` instead of `expect`.
        input.trim().parse().expect("Invalid age entered.")
    };

    let age = ask_for_age();
    println!("Hello, you are {} years old!", age);
}
