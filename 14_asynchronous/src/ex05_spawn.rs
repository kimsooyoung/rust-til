//! Topic 14 — `tokio::spawn` for fire-and-await background tasks.
//!
//! Run via: `cargo run --bin ex05_spawn`

use crate::helpers;
use tokio::time::{Duration, Instant, sleep};

async fn worker(id: u32, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("worker {id} slept {ms}ms")
}

async fn run_inner() {
    helpers::section("spawn basics");

    let started = Instant::now();

    // Each `spawn` returns a `JoinHandle<T>` and the task starts running
    // on the runtime's thread pool right away (concurrent, not sequential).
    let h1 = tokio::spawn(worker(1, 300));
    let h2 = tokio::spawn(worker(2, 500));
    let h3 = tokio::spawn(worker(3, 200));

    // `.await` on a `JoinHandle<T>` yields `Result<T, JoinError>`; the
    // `JoinError` would carry panic / cancellation info if it happened.
    let r1 = h1.await.expect("worker 1 panicked");
    let r2 = h2.await.expect("worker 2 panicked");
    let r3 = h3.await.expect("worker 3 panicked");

    println!("{r1}");
    println!("{r2}");
    println!("{r3}");

    // Total elapsed should be ~max(300, 500, 200) = ~500ms, not their sum,
    // because the three tasks run concurrently on the runtime.
    println!("elapsed: {:?}", started.elapsed());
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
