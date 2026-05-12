//! Topic 14 — `tokio::task::JoinSet` for completion-order task draining.
//!
//! Run via: `cargo run --bin ex16_joinset`

use crate::helpers;
use tokio::task::JoinSet;
use tokio::time::{Duration, Instant, sleep};

async fn run_inner() {
    helpers::section("JoinSet drains in completion order");

    // `JoinSet` beats a manual `Vec<JoinHandle<T>>` for two reasons:
    //   1. `join_next().await` yields whichever task finished *first*,
    //      not whichever handle is at index 0 — so we observe completion
    //      order without polling each handle ourselves.
    //   2. Dropping the `JoinSet` aborts every still-running task in it,
    //      giving you bulk cancellation by structured-concurrency scope.
    let mut set: JoinSet<u32> = JoinSet::new();

    // Spawn 5 tasks with deliberately out-of-order sleep durations.
    let timings: [(u32, u64); 5] = [(1, 400), (2, 100), (3, 250), (4, 50), (5, 300)];
    let started = Instant::now();
    for (id, ms) in timings {
        set.spawn(async move {
            sleep(Duration::from_millis(ms)).await;
            id
        });
    }

    while let Some(res) = set.join_next().await {
        let id = res.expect("task panicked");
        println!("task {id} finished at {:?}", started.elapsed());
    }
    println!("all tasks drained, total elapsed: {:?}", started.elapsed());
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
