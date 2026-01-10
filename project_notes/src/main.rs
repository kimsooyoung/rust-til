use std::env;
// this is for file operations (ex- .read, .write, .append, .create, .delete)
use std::fs::OpenOptions;
// this is for write_all(), read_to_string() etc.
use std::io::prelude::*;

use chrono::Local;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <filename>");
        std::process::exit(1);
    }
    
    // get current time information from chrono
    let now_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // get filename from command line arguments
    let filename = &args[1];
    println!("filename: {}", filename);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    // ? is for error handling, 
    // it propagates the error up to the main function
    file.write_all(b"<--")?;
    file.write_all(now_string.as_bytes())?;
    file.write_all(b"-->")?;
    file.write_all(b"\n\n")?;

    Ok(())
}
