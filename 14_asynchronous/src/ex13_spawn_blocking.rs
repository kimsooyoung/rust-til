//! Topic 14 — `tokio::task::spawn_blocking` for sync/blocking work.
//!
//! Run via: `cargo run --bin ex13_spawn_blocking`

use crate::helpers;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::time::{Duration, Instant, sleep};

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
    helpers::section("spawn_blocking: blocking pool runs CPU work; async futures still run");

    // While `blocking_sum` runs on a *blocking* thread, this task keeps waking
    // on an async worker — proof the runtime is not stuck inside the sum loop.
    let tick_count = Arc::new(AtomicU32::new(0));
    let ticks_for_ticker = Arc::clone(&tick_count);

    let ticker = tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(75)).await;
            let n = ticks_for_ticker.fetch_add(1, Ordering::Relaxed) + 1;
            println!(
                "  [ticker] tick {n} — async future ran while blocking_sum was on the blocking pool"
            );
        }
    });

    let started = Instant::now();
    let sum_handle = tokio::task::spawn_blocking(|| blocking_sum(50_000_000));
    let total = sum_handle.await.expect("blocking task panicked");
    let elapsed = started.elapsed();

    ticker.abort();
    let _ = ticker.await;

    let ticks = tick_count.load(Ordering::Relaxed);
    println!("sum(1..=50_000_000) = {total}");
    println!("elapsed: {elapsed:?}");
    println!(
        "ticker completed {ticks} ticks during the sum — other futures kept scheduling on async workers"
    );
    assert!(
        ticks >= 2,
        "expected multiple ticker wakes while sum ran; try lowering sleep interval or raising N if this fails on a very fast machine"
    );
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
