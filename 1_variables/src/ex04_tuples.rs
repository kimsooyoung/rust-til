//! Topic 4 — Tuples: indexing, destructuring, mutation, and iterating patterns.
//!
//! Run via: `cargo run --bin ex04_tuples`

use crate::helpers::section;

/// Homogeneous vs heterogeneous tuples, `mut` tuples, and iterating rows stored in a `Vec`.
pub fn run() {
    section("Tuples — access, iterate, edit");
    let pair = ("Rust", 2026);
    println!("tuple Debug: {pair:?}");
    println!("by index: language = {}, year = {}", pair.0, pair.1);
    let (lang, year) = pair;
    println!("destructured: {lang} {year}");

    let mut rgb = (255u8, 128u8, 0u8);
    rgb.1 = 200;
    println!("mut tuple after edit: {:?}", rgb);

    let triple = (1i32, 2, 3);
    let arr = [triple.0, triple.1, triple.2];
    print!("iterate (via array from homogeneous tuple): ");
    for v in arr {
        print!("{v} ");
    }
    println!();

    let rows = vec![("Alice", 10u32), ("Bob", 20), ("Carol", 30)];
    println!("iterate Vec of tuples:");
    for (name, value) in &rows {
        println!("  {name}: {value}");
    }

    println!("\nDone. Try uncommenting invalid lines in other topics to read compiler errors.");
}
