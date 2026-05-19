//! Binary entry point for the notes timestamp appender.
//!
//! Parses one positional argument (the target filename) and hands off to
//! [`project_notes::append_timestamp`].

use std::env;
use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <filename>");
        std::process::exit(1);
    }
    let filename = PathBuf::from(&args[1]);
    println!("filename: {}", filename.display());
    project_notes::append_timestamp(&filename)
}
