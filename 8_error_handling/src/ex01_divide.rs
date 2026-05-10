use std::error::Error;

use crate::fns::{divide_option, divide_result};

pub fn run() {
    let result = divide_option(10.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: Division by zero"),
    }

    let result = divide_result(10.0, 0.0);
    match result {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }

    let value: Result<&str, Box<dyn Error>> = Ok("This is Error Message!");
    match value {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }

    let value: Result<&str, ()> = Err(());
    match value {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error occurred"),
    }
}
