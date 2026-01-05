use std::future::Future;
use futures::executor::block_on;
use tokio::time::{sleep, Duration};

async fn get_name() -> String {
    "Sooyoung Kim".to_string()
}

// tokio usage
async fn call_api_one() -> String {
    sleep(Duration::from_secs(1)).await;
    "One".to_string()
}

async fn call_api_two() -> String {
    sleep(Duration::from_secs(1)).await;
    "Two".to_string()
}

fn return_future() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "Future".to_string()
    }
}

fn get_async_name() -> impl Future<Output = String> {
    let name = "Sooyoung Kim".to_string();
    async move { 
        sleep(Duration::from_secs(1)).await;
        format!("Async Name: {}", name) 
    }   
}

#[tokio::main]
async fn main() {
    // future - block_on()
    let name = block_on(get_name());
    println!("Name: {}", name);

    // tokio usage
    let one = call_api_one().await;
    let two = call_api_two().await;
    println!("One: {}", one);
    println!("Two: {}", two);

    // return future
    let future = return_future().await;
    println!("Future: {}", future);

    // get async name
    let async_name = get_async_name().await;
    println!("async_name: {}", async_name);
}