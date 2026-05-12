//! Topic 14 — `tokio::sync::mpsc` bounded multi-producer single-consumer channel.
//!
//! Run via: `cargo run --bin ex09_mpsc`

use std::time::Instant;

use crate::helpers;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

async fn run_inner() {
    helpers::section("bounded mpsc channel (capacity 4) — back-pressure visible");

    // Capacity 4 means `tx.send(..).await` will yield when 4 messages are
    // already queued and the consumer hasn't drained any yet — back-pressure
    // built into the channel itself.
    let (tx, mut rx) = mpsc::channel::<String>(4);

    // Watch the "waited" timings below: the first ~4 sends complete in
    // microseconds (slots are free), but once the channel is full each
    // subsequent `tx.send(..).await` blocks until the slow consumer
    // (sleeping 80ms per message) drains a slot. Expect waits to jump to
    // ~80ms once back-pressure kicks in.
    let producer = tokio::spawn(async move {
        for i in 1..=10 {
            let msg = format!("message #{i}");
            let started = Instant::now();
            tx.send(msg).await.expect("consumer dropped early");
            let waited = started.elapsed();
            println!("produced #{i} (waited {}ms)", waited.as_millis());
            // No `.await` between sends — push as fast as possible so the
            // channel fills and back-pressure takes over.
        }
        // `tx` is dropped at end of scope — that closes the channel and
        // unblocks the consumer's `recv()` with `None`.
    });

    while let Some(msg) = rx.recv().await {
        println!("received: {msg}");
        // Slow consumer: each receive takes ~80ms, which forces the
        // producer to wait once the 4-slot channel is full.
        sleep(Duration::from_millis(80)).await;
    }

    producer.await.expect("producer panicked");
    println!("channel closed cleanly");
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
