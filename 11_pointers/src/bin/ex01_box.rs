// Box<T> — heap allocation, single owner; use `*` to reach the inner `T`.
//
// Run: `cargo run --bin ex01_box`   (or `cargo run` if `default-run` is set to this crate binary)

fn main() {
    // Value lives on the heap; the stack holds the `Box` (pointer + metadata).
    let age = Box::new(22);
    // `*age` goes through `Deref` to read the `i32`; `i32` is `Copy`, so you get a copy.
    let twice = *age * 2;
    println!("twice: {}", twice);
}
