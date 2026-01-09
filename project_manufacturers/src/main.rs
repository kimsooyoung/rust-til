// cargo-watch -qc -x "run -- BMW" -x clippy

use std::env;

const API_URL: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";

struct Manufacturer<'a> {
    name: Option<&'a str>,
    common_name: Option<&'a str>,
    country: Option<&'a str>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get env argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <manufacturer>");
        return Ok(());
    }
    let manufacturer = &args[1];
    println!("Manufacturer: {}", manufacturer);
    Ok(())
}
