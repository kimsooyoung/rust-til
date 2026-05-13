//! Topic 14 — `async move` blocks capturing owned data for the returned future.
//!
//! Run via: `cargo run --bin ex04_async_move`

use std::future::Future;
use tokio::time::{Duration, sleep};

fn get_async_name() -> impl Future<Output = String> {
    let name = "Sooyoung Kim".to_string();
    async move {
        sleep(Duration::from_secs(1)).await;
        format!("Async Name: {}", name)
    }
}

async fn run_inner() {
    let async_name = get_async_name().await;
    println!("async_name: {}", async_name);
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
