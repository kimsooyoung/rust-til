//! Topic 3 — Unit-like struct (no fields): useful as markers or for trait-only types.
//!
//! Run via: `cargo run --bin ex03_unit_struct`

pub fn run() {
    #[derive(Debug)]
    struct UnitLikeStruct;
    let unit_like_struct = UnitLikeStruct;
    println!("unit_like_struct: {:?}", unit_like_struct);
}
