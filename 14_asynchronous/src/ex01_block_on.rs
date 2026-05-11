//! Topic 14 — driving a future to completion with `futures::executor::block_on`.
//!
//! Run via: `cargo run --bin ex01_block_on`

use futures::executor::block_on;

async fn get_name() -> String {
    "Sooyoung Kim".to_string()
}

pub fn run() {
    let name = block_on(get_name());
    println!("Name: {}", name);
}
