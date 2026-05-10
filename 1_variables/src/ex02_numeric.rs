//! Topic 2 — Integer widths, floats, and explicit casts when mixing numeric types.
//!
//! Run via: `cargo run --bin ex02_numeric`

use crate::helpers::section;

/// Prints several fixed-width integers, `f32`/`f64`, and safe casts between `i32` and `f64`.
pub fn run() {
    section("Integer types (signed / unsigned, widths)");
    let a: i8 = -128;
    let b: i16 = 1000;
    let c: i32 = -1;
    let d: i64 = 1_000_000;
    let e: i128 = i128::MAX;
    let f: isize = 7;

    let u: u8 = 255;
    let v: u16 = 65535;
    let w: u32 = 4_294_967_295;
    let x: u64 = 1 << 20;
    let y: u128 = u128::MAX;
    let z: usize = 1024;

    println!(
        "i8 {a}, i16 {b}, i32 {c}, i64 {d}, i128 {e}, isize {f}\n\
         u8 {u}, u16 {v}, u32 {w}, u64 {x}, u128 {y}, usize {z}"
    );

    section("Floating-point types");
    let pi32: f32 = std::f32::consts::PI;
    let pi64: f64 = std::f64::consts::PI;
    println!("f32 π ≈ {pi32:.7}, f64 π ≈ {pi64:.15}");

    section("Operators mixing numeric types (explicit casts)");
    let n: i32 = 10;
    let m: f64 = 3.0;
    // Rust never silently widens or mixes `i32` with `f64`; pick an explicit conversion.
    let sum = f64::from(n) + m;
    let diff = n - (m as i32);
    let bits: u8 = 0b1010;
    let shifted = bits << 1;
    let masked = shifted & 0b1111;
    println!("f64::from(i32) + f64: {sum}, i32 - f64 as i32: {diff}, bit play: {masked}");
}
