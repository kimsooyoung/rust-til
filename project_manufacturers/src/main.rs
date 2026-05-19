//! Binary entry point for the NHTSA manufacturer lookup CLI.
//!
//! Parses one positional argument (the search keyword) and hands off to
//! [`project_manufacturers::run`].

use std::env;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <manufacturer>");
        return Ok(());
    }
    project_manufacturers::run(&args[1]).await
}
