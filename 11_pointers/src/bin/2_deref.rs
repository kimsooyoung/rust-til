// Deref — makes a custom type act like a pointer; enables deref coercion (e.g. `&Wrapper` → `&T`).
//
// Run: `cargo run --bin 2_deref`

use std::ops::Deref;

// Demo wrapper: holds a `T` and implements `Deref` so it behaves a bit like `Box`.
struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

// With `Deref`, `*self` works, and `&BoxedValue<i32>` can coerce to `&i32` (deref coercion).
impl<T> Deref for BoxedValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// Takes `&i32`, not `&BoxedValue<i32>` — coercion supplies the `&i32`.
fn print_value(value: &i32) {
    println!("value: {}", value);
}

fn main() {
    // `deref()` vs `*`
    // - `deref()` returns `&T` to the inner value.
    // - `*x` roughly dereferences through `Deref`. For `i32` (`Copy`), that reads by copy.
    // - For non-`Copy` types like `String`, `*` behaves differently; learn those cases separately.
    let age_boxed = BoxedValue::new(22);
    let twice_boxed_deref = age_boxed.deref() * 2;
    let twice_boxed_ast = *age_boxed * 2;
    println!("twice_boxed_deref: {}", twice_boxed_deref);
    println!("twice_boxed_ast: {}", twice_boxed_ast);

    // `print_value` wants `&i32`. Passing `&BoxedValue<i32>` works: the compiler inserts `Deref`.
    let my_value = BoxedValue::new(10);
    print_value(&my_value);
}
