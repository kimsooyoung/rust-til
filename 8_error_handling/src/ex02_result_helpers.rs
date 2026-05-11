#![allow(clippy::unnecessary_unwrap)]

use crate::fns::{get_user_name, get_user_name_err};

pub fn run() {
    let my_name = get_user_name().expect("Error occurred");
    println!("My name is {}", my_name);

    let err_name = get_user_name_err().unwrap_or("[Error] No name provided");
    println!("My name is {}", err_name);

    let my_name_result = get_user_name();
    if my_name_result.is_ok() {
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
}
