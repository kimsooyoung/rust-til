//! Topic 14 — `tokio::sync::watch` for broadcasting the *latest* value.
//!
//! Run via: `cargo run --bin ex10_watch`

use crate::helpers;
use tokio::sync::watch;
use tokio::time::{Duration, sleep};

#[derive(Clone)]
enum State {
    Starting,
    Working(u32),
    Done(u32),
}

impl State {
    fn describe(&self) -> String {
        match self {
            State::Starting => "Starting".to_string(),
            State::Working(n) => format!("Working(step={n})"),
            State::Done(n) => format!("Done(step={n})"),
        }
    }
}

async fn subscriber(label: &'static str, mut rx: watch::Receiver<State>) {
    // `changed().await` returns `Err(_)` when every sender has been dropped,
    // which is our cue to exit. Each await wake gives us only the *latest*
    // value — intermediate updates can be skipped if we were slow.
    while rx.changed().await.is_ok() {
        let snapshot = rx.borrow_and_update().clone();
        println!("[{label}] saw state: {}", snapshot.describe());
        // Deliberately slow processing so the producer can publish several
        // states while we're still handling one. `watch` keeps only the
        // *latest* value, so we wake up to a coalesced state, not a queue.
        sleep(Duration::from_millis(80)).await;
    }
    println!("[{label}] sender dropped, exiting");
}

async fn run_inner() {
    helpers::section("watch channel: subscribers see only the latest value");

    let (tx, rx) = watch::channel(State::Starting);

    let s1 = tokio::spawn(subscriber("sub-A", rx.clone()));
    let s2 = tokio::spawn(subscriber("sub-B", rx));

    // Give subscribers a moment to park on their first `changed().await`.
    sleep(Duration::from_millis(20)).await;

    // BURST #1: pause after each `send` so slow subscribers (~80ms work +
    // sleep) can run `changed` → read → sleep between publishes. Each
    // `Working(n)` stays observable instead of being overwritten in one tick.
    const PAUSE_AFTER_SEND_MS: u64 = 100;
    for n in 1..=5 {
        tx.send(State::Working(n)).expect("subscribers gone");
        sleep(Duration::from_millis(PAUSE_AFTER_SEND_MS)).await;
    }

    // Let subscribers finish their last sleep before the rapid burst.
    sleep(Duration::from_millis(120)).await;

    // BURST #2: no `.await` between sends — slot jumps 6→…→9→`Done(9)` while
    // subscribers may still be in `sleep(80ms)`; they typically log once
    // (latest only), not every intermediate step.
    for n in 6..=9 {
        tx.send(State::Working(n)).expect("subscribers gone");
    }
    tx.send(State::Done(9)).expect("subscribers gone");

    // Drop the sender to close the channel and unblock subscribers' exit.
    drop(tx);

    s1.await.expect("sub-A panicked");
    s2.await.expect("sub-B panicked");
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
