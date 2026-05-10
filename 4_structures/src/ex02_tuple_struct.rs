//! Topic 2 — Tuple struct `Position`: a distinct type from `(i32, i32, i32)` with methods.
//!
//! Run via: `cargo run --bin ex02_tuple_struct`

use crate::position::Position;

pub fn run() {
    let position = Position(10, 20, 30);
    println!("position debug: {:?}", position);
    position.describe();
    let mut position_twice = position.twice();
    position_twice.describe();
    position_twice.make_twice();
    position_twice.describe();

    let position2 = Position::zero();
    let position3 = Position::zero();
    let position4 = Position::zero();

    position2.describe();
    position3.describe();
    position4.describe();
}
