//! Topic 14 — sequential `.await` on tokio futures inside a tokio runtime.
//!
//! Run via: `cargo run --bin ex02_sequential_await`

use tokio::time::{Duration, sleep};

async fn call_api_one() -> String {
    sleep(Duration::from_secs(1)).await;
    "One".to_string()
}

async fn call_api_two() -> String {
    sleep(Duration::from_secs(1)).await;
    "Two".to_string()
}

async fn run_inner() {
    let one = call_api_one().await;
    let two = call_api_two().await;
    println!("One: {}", one);
    println!("Two: {}", two);
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
