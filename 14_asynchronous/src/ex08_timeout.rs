//! Topic 14 — `tokio::time::timeout` to bound a future's wall-clock duration.
//!
//! Run via: `cargo run --bin ex08_timeout`

use crate::helpers;
use tokio::time::{Duration, sleep, timeout};

async fn slow_op() -> &'static str {
    sleep(Duration::from_secs(2)).await;
    "slow done"
}

async fn fast_op() -> &'static str {
    sleep(Duration::from_millis(50)).await;
    "fast done"
}

async fn run_inner() {
    helpers::section("timeout that elapses");

    // `timeout` returns `Result<T, Elapsed>` — the inner future is cancelled
    // (dropped) when the deadline fires.
    match timeout(Duration::from_millis(500), slow_op()).await {
        Ok(value) => println!("slow op finished in time: {value}"),
        Err(elapsed) => println!("slow op timed out: {elapsed}"),
    }

    helpers::section("timeout that succeeds");

    match timeout(Duration::from_secs(1), fast_op()).await {
        Ok(value) => println!("fast op finished in time: {value}"),
        Err(elapsed) => println!("fast op timed out: {elapsed}"),
    }
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
