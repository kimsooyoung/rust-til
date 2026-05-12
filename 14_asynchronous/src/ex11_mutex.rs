//! Topic 14 — `tokio::sync::Mutex` for shared state across `.await` points.
//!
//! Run via: `cargo run --bin ex11_mutex`

use crate::helpers;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

const TASKS: usize = 5;

async fn run_inner() {
    helpers::section("Arc<tokio::sync::Mutex<i32>> shared between tasks");

    // Prefer `tokio::sync::Mutex` over `std::sync::Mutex` *only* when the
    // lock guard must be held across `.await` points: the tokio guard is
    // `Send` and the lock yields cooperatively when contended. For purely
    // synchronous critical sections, `std::sync::Mutex` is faster.
    let counter = Arc::new(Mutex::new(0_i32));

    let mut handles = Vec::with_capacity(TASKS);
    for id in 0..TASKS {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            let mut guard = counter.lock().await;
            *guard += 1;
            // Hold the guard across an `.await` — this is the case where
            // `tokio::sync::Mutex` is required.
            sleep(Duration::from_millis(10)).await;
            println!("task {id} bumped counter to {}", *guard);
        }));
    }

    for h in handles {
        h.await.expect("worker panicked");
    }

    let final_value = *counter.lock().await;
    println!("final counter: {final_value}");
    assert_eq!(final_value, TASKS as i32);
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(run_inner());
}
