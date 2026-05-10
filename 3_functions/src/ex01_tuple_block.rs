//! Topic 1 — Tuple arguments, block expressions (last line is the value), and tuple returns.
//!
//! Run via: `cargo run --bin ex01_tuple_block`

use crate::fns::{divide_with_remainder, print_second_value};

pub fn run() {
    let my_tuple = (1, 2.0, true);
    let second = print_second_value(my_tuple);
    println!("returned second field: {second}");

    let usd_to_krw = {
        let usd = 50;
        usd * 1400
    };
    println!("USD 50 dollar is KRW {}", usd_to_krw);

    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} remainder {}", quotient, remainder);
}
