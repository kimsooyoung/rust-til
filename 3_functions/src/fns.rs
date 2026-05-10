//! Reusable functions for tuple demos and integer division with remainder.

/// Tuple is one compound value; with all `Copy` fields, the whole tuple is copied into the callee.
pub fn print_second_value(tup: (i32, f32, bool)) -> f32 {
    println!("second value: {}", tup.1);
    tup.1
}

/// Returns two `i32`s as a tuple — Rust’s way to return “multiple values”.
pub fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    assert_ne!(divisor, 0, "divisor must not be zero");
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}
