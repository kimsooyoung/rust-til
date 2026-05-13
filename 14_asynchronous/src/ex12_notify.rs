//! Topic 14 — `tokio::sync::Notify` for one-shot / broadcast wakeups.
//!
//! Run via: `cargo run --bin ex12_notify`

use crate::helpers;
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::time::{Duration, sleep};

async fn run_inner() {
    helpers::section("notify_waiters wakes everyone currently waiting");

    let notify = Arc::new(Notify::new());

    let n1 = Arc::clone(&notify);
    let waiter_a = tokio::spawn(async move {
        println!("[A] waiting");
        n1.notified().await;
        println!("[A] woke up");
    });

    let n2 = Arc::clone(&notify);
    let waiter_b = tokio::spawn(async move {
        println!("[B] waiting");
        n2.notified().await;
        println!("[B] woke up");
    });

    // Give both waiters a chance to register on the Notify.
    sleep(Duration::from_millis(50)).await;
    println!("[main] calling notify_waiters()");
    notify.notify_waiters();

    waiter_a.await.expect("A panicked");
    waiter_b.await.expect("B panicked");

    helpers::section("notify_one stores a single permit");

    // `notify_one` either wakes the longest-waiting waiter or stores a
    // single "permit" that the next `notified().await` will consume.
    let notify = Arc::new(Notify::new());

    let n1 = Arc::clone(&notify);
    let waiter_a = tokio::spawn(async move {
        println!("[A2] waiting");
        n1.notified().await;
        println!("[A2] woke up");
    });

    let n2 = Arc::clone(&notify);
    let waiter_b = tokio::spawn(async move {
        println!("[B2] waiting");
        n2.notified().await;
        println!("[B2] woke up");
    });

    sleep(Duration::from_millis(50)).await;
    println!("[main] calling notify_one() — only one of A2/B2 will wake");
    notify.notify_one();

    // Give one of them time to fire.
    sleep(Duration::from_millis(50)).await;
    println!("[main] calling notify_one() again to release the other");
    notify.notify_one();

    waiter_a.await.expect("A2 panicked");
    waiter_b.await.expect("B2 panicked");
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
