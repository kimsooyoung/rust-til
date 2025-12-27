// What is ownership?
// - Ownership is the concept that determines how a variable manages its data
// - Each value in Rust has an owner
// - The owner is responsible for the lifetime of the value
// - The value can be moved or copied, but the owner cannot be changed
// - The value can be borrowed, but the owner cannot be borrowed
// - The value can be borrowed mutably, but the owner cannot be borrowed mutably
// - The value can be borrowed immutably, but the owner cannot be borrowed immutably
// - The value can be borrowed mutably, but the owner cannot be borrowed mutably

// This moves the ownership of s to the get_str_len_with_ownership function
fn get_str_len_with_ownership(s: String) -> String {
    let str_len = s.len();
    println!("string length of {} is {}", s, str_len);

    s
}

// This borrows the ownership of s to the get_str_len_wo_ownership function
#[allow(clippy::ptr_arg)]
fn get_str_len_wo_ownership(s: &String) -> usize {
    s.len()
}

// This borrows the ownership of s with mutability
fn clear_string(s: &mut String) {
    s.clear();
}

// This does not work because s1 is already moved to _s1
// fn print_lost(s: &String) {
//     println!("string length of {} is {}", s1, s1.len());
// }

// This does not work because the return type is a borrowed value, but there is no value for it to be borrowed from
// fn get_name() -> &String {
//     &"Sooyoung".to_string()
// }

// This allows the main function to compile even with the warning.
#[allow(warnings)]
fn main() {
    // String is a reference type
    let s1 = String::from("Hello Rust!");

    // This moves the ownership of s1 to the get_str_len_with_ownership function
    let s1 = get_str_len_with_ownership(s1);

    // This borrows the ownership of s1 to the get_str_len_wo_ownership function
    let str_len2 = get_str_len_wo_ownership(&s1);
    println!("string length of {} is {}", s1, str_len2);

    let _s1 = s1;
    // This does not work because s1 is already moved to _s1
    // println!("string length of {} is {}", s1, str_len);
    println!("string length of {} is {}", _s1, str_len2);

    // This does not work because by default, variables in Rust are immutable.
    // When you declare `let a: i32 = 5;`, `a` cannot be changed.
    // Therefore, trying to assign `a = 10;` will cause a compile-time error.
    // let a: i32 = 5;
    // a = 10;
    // println!("a: {}", a);

    let mut a: i32 = 5;
    a = 10;
    println!("a: {}", a);

    let mut name: String = String::from("Sooyoung");
    println!("name: {}", name);
    clear_string(&mut name);
    println!("name: {}", name);

    // This does not work because const is immutable by default.
    // const A: i32 = 5;
    // A = 10;
    // println!("A: {}", A);

    // This works because mut is mutable by default.
    // const mut A: i32 = 5;

    const A: i32 = 5;
    println!("A: {}", A);

    // No mutable reference can be created after a mutable or immutable reference is created
    let mut b: i32 = 5;
    let ref_b: &mut i32 = &mut b;
    // let ref_b2: &i32 = &b;
    println!("ref_b: {}", ref_b);
    // println!("ref_b2: {}", ref_b2);

    // =========================================================================
    // Shadowing
    // =========================================================================

    let x: i32 = 5;
    println!("x: {}", x);
    let x = x + 1;
    println!("x: {}", x);
    let x = x * 2;
    println!("x: {}", x);
}
