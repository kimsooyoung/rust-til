//! Topic 14 — `tokio::select!` for racing concurrent futures.
//!
//! Run via: `cargo run --bin ex06_select`

use crate::helpers;
use tokio::time::{Duration, sleep};

async fn fast() -> &'static str {
    sleep(Duration::from_millis(200)).await;
    "fast"
}

async fn slow() -> &'static str {
    sleep(Duration::from_secs(1)).await;
    "slow"
}

async fn run_inner() {
    helpers::section("select! races two futures");

    // `select!` polls every arm concurrently and resolves with the first
    // one ready. The unselected arm's future is dropped at the end of the
    // macro — i.e. it is *cancelled* (its destructor runs and any work
    // in flight is abandoned at the next `.await` point).
    let winner = tokio::select! {
        s = fast() => s,
        s = slow() => s,
    };
    println!("winner: {winner}");

    helpers::section("biased select: two arms ready at the same instant");

    // Without `biased;`, `select!` randomizes the poll order across arms
    // that are simultaneously ready, so the winner of a tie is not
    // deterministic. With `biased;`, arms are polled top-to-bottom in
    // source order, so when several arms are ready in the same poll the
    // first one in the source wins.
    //
    // Both arms below complete at the *same* `Duration` (200ms), so they
    // are ready in the same poll. We run the `select!` three times to
    // show that with `biased;` arm-A wins deterministically every time.
    for round in 1..=3 {
        let arm = tokio::select! {
            biased;
            _ = sleep(Duration::from_millis(200)) => "arm-A (first in source)",
            _ = sleep(Duration::from_millis(200)) => "arm-B (second in source)",
        };
        println!("biased round {round}: {arm} fired");
    }

    // For contrast: without `biased;` the runtime randomises tie-breaks,
    // so over several rounds we should see *both* arms win at least once
    // (in practice this is overwhelmingly likely across 6 rounds).
    let mut a_wins = 0_u32;
    let mut b_wins = 0_u32;
    for _ in 0..6 {
        let arm = tokio::select! {
            _ = sleep(Duration::from_millis(50)) => 'A',
            _ = sleep(Duration::from_millis(50)) => 'B',
        };
        if arm == 'A' {
            a_wins += 1;
        } else {
            b_wins += 1;
        }
    }
    println!("unbiased 6 rounds (random tie-break): A={a_wins} B={b_wins}");
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
