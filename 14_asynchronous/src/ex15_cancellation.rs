//! Topic 14 — Cooperative cancellation with `watch` + `select!`.
//!
//! Run via: `cargo run --bin ex15_cancellation`
//!
//! The canonical higher-level primitive for this pattern is
//! `tokio_util::sync::CancellationToken`, but that lives in the separate
//! `tokio-util` crate. This example sticks to pure-tokio primitives so it
//! works against the `tokio` crate alone: a `watch::channel::<bool>` acts
//! as a shutdown flag, and workers use `select!` to race their own work
//! against `shutdown.changed()`.

use crate::helpers;
use tokio::sync::watch;
use tokio::time::{Duration, interval, sleep};

async fn worker(id: u32, mut shutdown: watch::Receiver<bool>) {
    let mut ticker = interval(Duration::from_millis(150));
    let mut iterations = 0_u32;
    loop {
        tokio::select! {
            // The `shutdown.changed()` arm fires the moment the sender
            // publishes a new value — we then check the flag and exit.
            res = shutdown.changed() => {
                if res.is_err() || *shutdown.borrow_and_update() {
                    println!("[worker {id}] shutdown received after {iterations} ticks");
                    break;
                }
            }
            _ = ticker.tick() => {
                iterations += 1;
                println!("[worker {id}] doing work (tick {iterations})");
            }
        }
    }
}

async fn run_inner() {
    helpers::section("cooperative cancellation via watch<bool>");

    let (tx, rx) = watch::channel(false);

    let w1 = tokio::spawn(worker(1, rx.clone()));
    let w2 = tokio::spawn(worker(2, rx));

    // Let the workers run for a bit, then ask them to stop.
    sleep(Duration::from_millis(600)).await;
    println!("[main] requesting shutdown");
    tx.send(true).expect("workers gone before shutdown signal");

    w1.await.expect("worker 1 panicked");
    w2.await.expect("worker 2 panicked");
    println!("[main] all workers exited");
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
