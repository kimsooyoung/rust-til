// // Approach 1
// enum Option<T> {
//     Some(T),
//     None,
// }

// // Approach 2
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Approach 1
fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
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
}
