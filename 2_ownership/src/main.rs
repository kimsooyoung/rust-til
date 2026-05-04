// =========================================================================
// Ownership
// =========================================================================
// - Every value has exactly one owner at a time.
// - When the owner goes out of scope, the value is `drop`ped (cleaned up).
// - `Copy` types (e.g. `i32`, `bool`) copy on assignment; most others *move* on assignment
//   or when passed by value (e.g. `String` into `fn f(s: String)`).
// - A *borrow* is a reference (`&T` immutable, `&mut T` mutable): use data without taking
//   ownership from the caller.
// - Borrowing rule: at any point, either any number of `&T`, *or* one `&mut T` to the same
//   data — not both overlapping in the same lifetime.

// Takes ownership of `s`, uses it, then returns it so the caller owns the `String` again.
fn get_str_len_with_ownership(s: String) -> String {
    let str_len = s.len();
    println!("string length of {} is {}", s, str_len);

    s
}

// Immutable borrow: only reads `s`; caller keeps the owned `String`.
// Call sites may pass `&String` or `&str`; both coerce to `&str` where needed.
fn get_str_len_wo_ownership(s: &str) -> usize {
    s.len()
}

// Mutable borrow: mutates the buffer in place; caller still owns the `String`.
fn clear_string(s: &mut String) {
    s.clear();
}

// Prints length using the parameter you actually received (`s`).
// (A common beginner bug is to use a different name here, e.g. `s1`, which does not exist.)
fn print_str_len(s: &str) {
    println!("print_str_len: {} (len {})", s, s.len());
}

// Returns an owned `String`: the caller owns the allocation.
// You cannot return `&String` to a temporary from `"text".to_string()` — that value is
// dropped at the end of the function, so the reference would dangle.
fn get_name_owned() -> String {
    "Sooyoung".to_string()
}

// String literals have `'static` lifetime; returning `&'static str` is always safe.
fn get_name_static() -> &'static str {
    "Sooyoung"
}

fn main() {
    // ---------------------------------------------------------------------
    // Owned `String` (heap buffer). This is not a “reference type”; `&str` / `&String` are.
    // ---------------------------------------------------------------------
    let s1 = String::from("Hello Rust!");

    // `s1` is moved into the function; the returned `String` is bound to `s1` again.
    let s1 = get_str_len_with_ownership(s1);

    // Borrow for a read: `s1` remains valid for the rest of this block.
    let str_len2 = get_str_len_wo_ownership(&s1);
    println!("string length of {} is {}", s1, str_len2);

    // Move `s1` into `_s1`. The name `s1` is no longer valid after this line.
    let _s1 = s1;
    // println!("string length of {} is {}", s1, str_len2); // error: use of moved `s1`
    println!("string length of {} is {}", _s1, str_len2);
    print_str_len(&_s1);

    let owned_name = get_name_owned();
    println!("get_name_owned: {owned_name}");
    let static_name = get_name_static();
    println!("get_name_static: {static_name}");

    // ---------------------------------------------------------------------
    // `mut` bindings vs reassignment
    // ---------------------------------------------------------------------
    // Without `mut`, the binding cannot be reassigned:
    // let a: i32 = 5;
    // a = 10; // error

    let mut a: i32 = 5;
    println!("a before reassignment: {a}");
    a = 10;
    println!("a after reassignment: {a}");

    let mut name: String = String::from("Sooyoung");
    println!("name: {}", name);
    clear_string(&mut name);
    println!("name: {}", name);

    // ---------------------------------------------------------------------
    // `const`: compile-time constant; no reassignment, and no `const mut` in Rust.
    // ---------------------------------------------------------------------
    // const A: i32 = 5;
    // A = 10; // error

    const A: i32 = 5;
    println!("A: {}", A);

    // ---------------------------------------------------------------------
    // Non-overlapping borrows: `&mut b` ends at the closing `}`, then `&b` is allowed.
    // ---------------------------------------------------------------------
    let mut b: i32 = 5;
    {
        let ref_b: &mut i32 = &mut b;
        println!("ref_b: {}", ref_b);
    }
    let ref_b2: &i32 = &b;
    println!("ref_b2: {}", ref_b2);

    // ---------------------------------------------------------------------
    // Shadowing: each `let x = ...` introduces a new binding (name reused).
    // ---------------------------------------------------------------------
    let x: i32 = 5;
    println!("x: {}", x);
    let x = x + 1;
    println!("x: {}", x);
    let x = x * 2;
    println!("x: {}", x);
}
