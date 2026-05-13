//! Topic 14 — returning `impl Future<Output = T>` from a synchronous function.
//!
//! Run via: `cargo run --bin ex03_return_future`

use std::future::Future;
use tokio::time::{Duration, sleep};

#[allow(clippy::manual_async_fn)]
fn return_future() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "Future".to_string()
    }
}

async fn run_inner() {
    let future = return_future().await;
    println!("Future: {}", future);
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
