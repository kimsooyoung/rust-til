//! Topic 14 — Building a custom tokio runtime with `runtime::Builder`.
//!
//! Run via: `cargo run --bin ex14_runtime_builder`

use crate::helpers;
use tokio::time::{Duration, sleep};

async fn announce(id: u32) {
    let name = std::thread::current()
        .name()
        .unwrap_or("<unnamed>")
        .to_string();
    sleep(Duration::from_millis(50)).await;
    println!("task {id} running on thread '{name}'");
}

pub fn run() {
    helpers::section("custom runtime via Builder::new_multi_thread");

    // Unlike the other examples in this chapter, we don't use
    // `Runtime::new()` — that would give us defaults. Here we construct
    // the runtime explicitly so we can tune worker count, thread naming,
    // and which drivers are enabled.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("ex14-worker")
        .enable_all() // timer + I/O drivers
        .build()
        .expect("failed to build custom tokio runtime");

    rt.block_on(async {
        let h1 = tokio::spawn(announce(1));
        let h2 = tokio::spawn(announce(2));
        let h3 = tokio::spawn(announce(3));
        for h in [h1, h2, h3] {
            h.await.expect("spawned task panicked");
        }
    });
}
