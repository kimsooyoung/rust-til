fn print_second_value(tup: (i32, f32, bool)) -> f32 {
    println!("second value: {}", tup.1);
    tup.1
}

/// Multiple return values using tuples
/// - Functions can return multiple values by wrapping them in a tuple
/// - Callers can use destructuring to unpack the returned values
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

fn main() {
    let my_tuple = (1, 2.0, true);
    print_second_value(my_tuple);

    let usd_to_krw = {
        let usd = 50;
        usd * 1400
    };
    println!("usd_to_krw: {}", usd_to_krw);

    // Multiple return values using tuples
    // Destructuring the tuple into separate variables
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} remainder {}", quotient, remainder);

    // inline function
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
