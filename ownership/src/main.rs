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

fn main() {
    // String is a reference type
    let s1 = String::from("Hello Rust!");
    let str_len = get_str_len(&s1);
    println!("string length of {} is {}", s1, str_len);

    let _s1 = s1;
    // This does not work because s1 is already moved to _s1
    // println!("string length of {} is {}", s1, str_len);
    println!("string length of {} is {}", _s1, str_len);
}
