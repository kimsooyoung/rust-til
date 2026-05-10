//! Topic 2 — Closures and a tiny stdin example (`read_line` + `parse`).
//!
//! Run via: `cargo run --bin ex02_closures` (expects interactive input for age).

pub fn run() {
    let greeting = |name: &str| format!("Hello, {}!", name);
    println!("greeting: {}", greeting("Sooyoung"));

    let ask_for_age = || -> u32 {
        use std::io::{self, Write};

        print!("Enter your age: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().expect("Invalid age entered.")
    };

    let age = ask_for_age();
    println!("Hello, you are {} years old!", age);
}
