// Cell<T> — mutate a small `Copy` value through `&self` (no runtime borrow counter).
// Sharing across threads would need `Mutex` / atomics etc.; this sample is single-threaded.
//
// Run: `cargo run --bin 4_cell`

use std::cell::Cell;

struct Person {
    name: String,
    // Only `age` is inside `Cell`, so you can bump the number while holding `&Person`.
    age: Cell<i32>,
}

impl Person {
    // `&self` is enough because `Cell` allows get/set on the inner `i32`.
    fn increase_age(&self) -> i32 {
        self.age.set(self.age.get() + 1);
        self.age.get()
    }
}

fn main() {
    let me = Person {
        name: String::from("Sooyoung"),
        age: Cell::new(28),
    };
    println!("{} before age bump", me.name);
    let age = me.increase_age();
    println!("age: {}", age);
}
