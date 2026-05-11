#![allow(clippy::unnecessary_literal_unwrap)]

pub fn run() {
    let name: Option<&str> = Some("Option");
    match name {
        Some(name) => println!("name: {}", name),
        None => println!("name is None"),
    }

    let mut age: Option<i32> = Some(20);
    match age.as_mut() {
        Some(age) => *age += 1,
        None => println!("age is None"),
    }
    println!("Your Korean age is: {}", age.unwrap());

    let age1: Option<i32> = Some(20);
    let age2: Option<i32> = Some(30);
    let age3: Option<i32> = Some(40);
    if let (Some(age1), Some(age2), Some(age3)) = (age1, age2, age3) {
        println!("Age sum: {}", age1 + age2 + age3);
    }

    let your_name: Option<&str> = None;
    let unwrapped_your_name = your_name.unwrap_or("Hong Gil Dong");
    println!("unwrapped_your_name: {}", unwrapped_your_name);

    let your_name: Option<&str> = None;
    let unwrapped_your_name = your_name.unwrap_or_else(|| {
        println!("No name provided");
        "Hong Gil Dong"
    });
    println!("unwrapped_your_name: {}", unwrapped_your_name);

    let son_name: Option<&str> = Some("Hong Gil Dong");
    if son_name.is_some() {
        println!("son_name is some");
    } else {
        println!("son_name is none");
    }

    let bus_ticket_price: Option<i32> = None;
    let default_price = bus_ticket_price.unwrap_or_default();
    println!("default_price: {}", default_price);

    let illegal_bus_ticket_price: Option<i32> = Some(1000);
    let price = illegal_bus_ticket_price
        .map(|price| price * 200)
        .unwrap_or_default();
    println!("illegal bus ticket price: {}", price);
}
