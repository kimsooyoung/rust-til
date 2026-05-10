// RefCell<T> — enforces `&` / `&mut` rules at runtime; breaking them panics (not undefined behavior).
//
// Run: `cargo run --bin 5_refcell`

use std::cell::RefCell;

fn main() {
    // Borrow rules checked at runtime, not only at compile time.
    // While `borrow_mut()` is active, you must not call `borrow()` — overlapping borrows panic.
    // Use a `{ ... }` block so `RefMut` drops before the next borrow.
    let ref_cell = RefCell::new(vec![1, 2, 3]);

    {
        let mut mutable_ref = ref_cell.borrow_mut();
        mutable_ref.push(10);
        println!("mutable_ref: {:?}", mutable_ref);
    } // `RefMut` drops here; the mutable borrow ends.

    // Now a shared borrow is OK — e.g. read-only `len()`.
    let len = ref_cell.borrow().len();
    println!("len: {}", len);

    // Uncomment both lines together to trigger a runtime panic (mutable + immutable overlap).
    // let _m = ref_cell.borrow_mut();
    // let _i = ref_cell.borrow();
}
