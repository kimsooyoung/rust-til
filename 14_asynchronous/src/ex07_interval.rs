//! Topic 14 — `tokio::time::interval` for periodic loops.
//!
//! Run via: `cargo run --bin ex07_interval`

use crate::helpers;
use tokio::time::{Duration, MissedTickBehavior, interval};

async fn run_inner() {
    helpers::section("interval ticking every 200ms");

    // Default behavior is `MissedTickBehavior::Burst`: if a tick is missed
    // (the loop body took too long), `interval` fires the missed ticks
    // back-to-back to "catch up" the schedule.
    let mut ticker = interval(Duration::from_millis(200));

    // For most "do work every N" loops you want `Delay` instead: a missed
    // tick is simply rescheduled `N` from now, so we never burst.
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

    for n in 1..=5 {
        // The very first `.tick().await` returns immediately (tick at t=0).
        ticker.tick().await;
        println!("tick #{n}");
    }
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
