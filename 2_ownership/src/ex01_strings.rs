//! Topic 1 — Owned `String`, moving into functions, and borrowing for reads.
//!
//! Run via: `cargo run --bin ex01_strings`

use crate::fns::{
    get_name_owned, get_name_static, get_str_len_wo_ownership, get_str_len_with_ownership,
    print_str_len,
};

pub fn run() {
    let s1 = String::from("Hello Rust!");
    let s1 = get_str_len_with_ownership(s1);

    let str_len2 = get_str_len_wo_ownership(&s1);
    println!("string length of {} is {}", s1, str_len2);

    let _s1 = s1;
    println!("string length of {} is {}", _s1, str_len2);
    print_str_len(&_s1);

    let owned_name = get_name_owned();
    println!("get_name_owned: {owned_name}");
    let static_name = get_name_static();
    println!("get_name_static: {static_name}");
}
