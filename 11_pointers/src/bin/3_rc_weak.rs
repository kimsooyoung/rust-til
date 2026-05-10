// Rc<T> — shared ownership on one thread via reference counting.
// Weak — non-owning handle; `upgrade()` returns `None` after the value is dropped (helps break cycles).
//
// Run: `cargo run --bin 3_rc_weak`

use std::rc::Rc;

fn main() {
    // Several `Rc`s can point at the same allocation; strong count tracks owners.
    let arr = vec!["Hello".to_string(), "World".to_string(), "Rust".to_string()];
    let rc = Rc::new(arr);

    // `Weak` does not increment the strong count — when the last `Rc` goes away, data is freed.
    let weak = Rc::downgrade(&rc);
    drop(rc); // strong count hits 0 here, so the `Vec` is dropped.

    // After drop, `upgrade()` returns `None` (we use `assert!` instead of `unwrap()` to avoid panic).
    assert!(weak.upgrade().is_none());

    // `clone()` on `Rc` duplicates the pointer and bumps the count — it does not deep-clone the `Vec`.
    let new_arr = vec!["Hello".to_string(), "World".to_string(), "Rust".to_string()];
    let rc = Rc::new(new_arr);
    let rc2 = rc.clone();
    drop(rc); // Only one handle dropped; `rc2` still keeps the data alive.
    println!("rc2: {:?}", rc2);
}
