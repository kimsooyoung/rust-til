// Conceptual shapes:
// Option<T>  -> Some(T) | None
// Result<T,E> -> Ok(T) | Err(E)
//
// Use Option when "missing value" is enough.
// Use Result when you also want error details (type E).

use std::error::Error;

fn get_user_name() -> Result<&'static str, ()> {
    Ok("Sooyoung")
}

fn get_user_name_err() -> Result<&'static str, ()> {
    Err(())
}

fn get_first_name() -> Result<String, ()> {
    Ok("Sooyoung".to_string())
}

fn get_last_name() -> Result<String, ()> {
    // Switch to Ok("Kim".to_string()) to see success in get_full_name().
    Err(())
}

fn get_full_name() -> Result<String, ()> {
    // `?` means: if Err, return early from this function with that Err.
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

// Approach 1: Option for "value may be absent".
fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Approach 2: Result when you want an error message.
fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    // -------------------------------------------------------------------------
    // Option vs Result for division
    // -------------------------------------------------------------------------
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

    // Result can use a trait object error type as well.
    let value = Result::<&str, Box<dyn Error>>::Ok("This is Error Message!");
    match value {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }

    // Hide the concrete error value by matching Err(_).
    let value: Result<&str, ()> = Err(());
    match value {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error occurred"),
    }

    // Expecting a value from Result: panics on Err with custom message.
    let my_name = get_user_name().expect("Error occurred");
    println!("My name is {}", my_name);

    // Fallback value on Err (no panic).
    let err_name = get_user_name_err().unwrap_or("[Error] No name provided");
    println!("My name is {}", err_name);

    // is_ok / is_err checks
    let my_name_result = get_user_name();
    if my_name_result.is_ok() {
        // Safe here because we just checked is_ok().
        println!("My name is {}", my_name_result.unwrap());
    } else {
        println!("Error occurred");
    }

    let err_name_result = get_user_name_err();
    if err_name_result.is_err() {
        println!("Error occurred");
    } else {
        println!("My name is {}", err_name_result.unwrap());
    }

    // Early exit with `?` already happened inside get_full_name().
    let full_name = get_full_name();
    match full_name {
        Ok(x) => println!("Full name: {}", x),
        Err(_) => println!("Error occurred"),
    }

    // unwrap_or_default with Result<T, E>: on Err, returns T::default().
    let full_name = get_full_name();
    let full_name_len = full_name.map(|x| x.len()).unwrap_or_default();
    println!("Full name length: {}", full_name_len);

    // map_err transforms only the error side.
    let full_name = get_full_name();
    let full_name_err = full_name.map_err(|_| "Error during map");
    match full_name_err {
        Ok(x) => println!("Full name: {}", x),
        Err(e) => println!("Error: {}", e),
    }
}
