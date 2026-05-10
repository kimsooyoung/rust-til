//! Topic 3 — `const`, non-overlapping `&mut` scopes, and shadowing with `let`.
//!
//! Run via: `cargo run --bin ex03_borrow_shadow`

pub fn run() {
    const A: i32 = 5;
    println!("A: {}", A);

    let mut b: i32 = 5;
    {
        let ref_b: &mut i32 = &mut b;
        println!("ref_b: {}", ref_b);
    }
    let ref_b2: &i32 = &b;
    println!("ref_b2: {}", ref_b2);

    let x: i32 = 5;
    println!("x: {}", x);
    let x = x + 1;
    println!("x: {}", x);
    let x = x * 2;
    println!("x: {}", x);
}
