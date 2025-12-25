// What is ownership?
// - Ownership is the concept that determines how a variable manages its data
// - Each value in Rust has an owner
// - The owner is responsible for the lifetime of the value
// - The value can be moved or copied, but the owner cannot be changed
// - The value can be borrowed, but the owner cannot be borrowed
// - The value can be borrowed mutably, but the owner cannot be borrowed mutably
// - The value can be borrowed immutably, but the owner cannot be borrowed immutably
// - The value can be borrowed mutably, but the owner cannot be borrowed mutably

fn get_str_len(s: &String) -> usize {
    s.len()
}

// This does not work because s1 is already moved to _s1
// fn print_lost(s: &String) {
//     println!("string length of {} is {}", s1, s1.len());
// }


// This allows the main function to compile even with the warning.
#[allow(warnings)]
fn main() {
    // String is a reference type
    let s1 = String::from("Hello Rust!");
    let str_len = get_str_len(&s1);
    println!("string length of {} is {}", s1, str_len);

    let _s1 = s1;
    // This does not work because s1 is already moved to _s1
    // println!("string length of {} is {}", s1, str_len);
    println!("string length of {} is {}", _s1, str_len);

    // This does not work because by default, variables in Rust are immutable.
    // When you declare `let a: i32 = 5;`, `a` cannot be changed.
    // Therefore, trying to assign `a = 10;` will cause a compile-time error.
    // let a: i32 = 5;
    // a = 10;
    // println!("a: {}", a);

    let mut a: i32 = 5;
    a = 10;
    println!("a: {}", a);

    // This does not work because const is immutable by default.
    // const A: i32 = 5;
    // A = 10;
    // println!("A: {}", A);

    // This works because mut is mutable by default.
    // const mut A: i32 = 5;

    const A: i32 = 5;
    println!("A: {}", A);


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
