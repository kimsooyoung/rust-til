//! Topic 2 — `mut` bindings vs reassignment, and mutating a `String` through `&mut`.
//!
//! Run via: `cargo run --bin ex02_mut_clear`

use crate::fns::clear_string;

pub fn run() {
    let mut a: i32 = 5;
    println!("a before reassignment: {a}");
    a = 10;
    println!("a after reassignment: {a}");

    let mut name: String = String::from("Sooyoung");
    println!("name: {}", name);
    clear_string(&mut name);
    println!("name: {}", name);
}
