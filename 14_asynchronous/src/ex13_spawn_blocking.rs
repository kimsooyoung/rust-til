//! Topic 14 — `tokio::task::spawn_blocking` for sync/blocking work.
//!
//! Run via: `cargo run --bin ex13_spawn_blocking`

use crate::helpers;
use tokio::time::Instant;

fn blocking_sum(n: u64) -> u64 {
    // Pretend this is a CPU-heavy or sync-I/O call (e.g. a `rusqlite` query
    // or `std::fs::read`). Running this directly inside an `async fn` would
    // stall the runtime worker thread that polls it, blocking *every other*
    // task scheduled on that worker until the function returns.
    //
    // `spawn_blocking` moves the closure to tokio's dedicated blocking
    // thread pool so the async workers stay responsive.
    (1..=n).sum()
}

async fn run_inner() {
    helpers::section("spawn_blocking offloads sync work");

    let started = Instant::now();
    let handle = tokio::task::spawn_blocking(|| blocking_sum(50_000_000));
    let total = handle.await.expect("blocking task panicked");

    println!("sum(1..=50_000_000) = {total}");
    println!("elapsed: {:?}", started.elapsed());
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
