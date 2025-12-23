fn print_string(s: &str) {
    println!("print_string: {}", s);
}

fn main() {
    // signed / unsigned integers
    let x: i64 = 100;
    let y: u64 = 100;
    println!("signed x: {}, unsigned y: {}", x, y);

    // Array [T; N]
    // - Fixed size at compile time (size N is part of the type)
    // - Stored on the stack and owns its data
    // - [i32; 3] and [i32; 5] are different types
    let num_arr: [i32; 3] = [1, 2, 3];
    println!("num_arr: {:?}", num_arr);
    
    // Mix array is not allowed
    // let mix_arr = [1, 2, 3, "Hello", true];
    // println!("mix_arr: {:?}", mix_arr);

    let str_arr: [&str; 3] = ["Hello", "World", "Rust"];
    println!("str_arr: {:?}", str_arr);

    let str_arr: [String; 3] = ["Hello".to_string(), "World".to_string(), "Rust".to_string()];
    println!("str_arr: {:?}", str_arr);

    // Tuple
    let mix_tup: (i32, f32, bool) = (1, 2.0, true);
    println!("mix_tup: {:?}", mix_tup);

    // Slices &[T]
    // - Dynamically sized (size is NOT part of the type)
    // - Always a reference (borrowed view) into existing data
    // - Stored as a "fat pointer" (pointer + length) on the stack
    // - More flexible for function params: fn foo(nums: &[i32]) accepts any array size
    let num_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("num_slice: {:?}", num_slice);

    let str_slice: &[&str] = &["Hello", "World", "Rust"];
    println!("str_slice: {:?}", str_slice);

    // String vs String Slice (&str)
    let mut my_string = String::from("Hello, world!");
    my_string.push_str(", Rust!");
    println!("my_string: {}", my_string);

    let my_string_slice: &str = &my_string[0..5];
    // my_string.push_str(", Rust!");
    print_string(my_string_slice);
}
