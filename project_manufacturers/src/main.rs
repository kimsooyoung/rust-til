// cargo-watch -qc -x "run -- BMW" -x clippy

use std::env;
use serde_json::Value;

// Type alias to make our code cleaner and easier to read
type ApiResponse = Value;

// The NHTSA API endpoint that provides vehicle manufacturer data
const API_URL: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";

// Manufacturer struct to hold information about each manufacturer
// 
// Why 'a (lifetime parameter) here?
// Because the API returns strings, and we want to store references to those strings
// instead of copying them. This is more memory efficient than using String.
// The lifetime 'a ensures these references are valid as long as the data they point to exists.
struct Manufacturer<'a> {
    name: Option<&'a str>,           // The official manufacturer name
    common_name: Option<&'a str>,     // The commonly used name (e.g., "BMW" instead of "Bayerische Motoren Werke")
    country: Option<&'a str>,         // The country where the manufacturer is based
}

// Trait to check if a manufacturer matches a search keyword
// This allows us to search across multiple fields (name, common_name, country)
trait Contains {
    fn contains(&self, name: &str) -> bool;
}

// Implementation of the Contains trait for Manufacturer
// This checks if the keyword appears in any of the manufacturer's fields
impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, name: &str) -> bool {
        // Check if the keyword appears in the name, common_name, or country
        // unwrap_or_default() safely handles None values by using an empty string
        self.name.unwrap_or_default().contains(name) 
            || self.common_name.unwrap_or_default().contains(name) 
            || self.country.unwrap_or_default().contains(name)
    }
}

// Implementation block for Manufacturer with helper methods
impl<'a> Manufacturer<'a> {
    // Pretty print all the manufacturer information in a readable format
    fn print_description(&self) {
        println!("Manufacturer: {}", self.name.unwrap_or_default());
        println!("Common Name: {}", self.common_name.unwrap_or_default());
        println!("Country: {}", self.country.unwrap_or_default());
    }
}

// Main function - this is where our program starts!
// The #[tokio::main] attribute sets up an async runtime so we can make HTTP requests
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Get the search keyword from command line arguments
    // The user provides this when running: cargo run -- BMW
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <manufacturer>");
        return Ok(());
    }
    let keyword = &args[1];
    println!("Keyword: {}", keyword);

    // Step 2: Make an HTTP GET request to the NHTSA API
    // The ? operator propagates errors up if something goes wrong
    let response: ApiResponse = reqwest::get(API_URL)
        .await?                    // Wait for the HTTP request to complete
        .json::<serde_json::Value>()  // Parse the response body as JSON
        .await?;                   // Wait for JSON parsing to complete

    // Step 3: Extract the manufacturers array from the JSON response
    // The API returns: { "Results": [ {...}, {...}, ... ] }
    let manufacturers_array = response
        .as_object()               // Treat the response as a JSON object
        .unwrap()
        .get("Results")            // Get the "Results" field
        .unwrap()
        .as_array()                // Treat it as a JSON array
        .unwrap();

    // Step 4: Search through all manufacturers and find matches
    let mut found_any = false;     // Track if we found any matches
    for item in manufacturers_array {
        // Extract individual fields from each manufacturer object
        let obj = item.as_object().unwrap();
        let country = obj.get("Country").and_then(|v| v.as_str());
        let name = obj.get("Mfr_Name").and_then(|v| v.as_str());
        let common_name = obj.get("Mfr_CommonName").and_then(|v| v.as_str());
        
        // Create a Manufacturer struct with the extracted data
        let manufacturer = Manufacturer {
            name,
            common_name,
            country,
        };
        
        // Check if this manufacturer matches our search keyword
        if manufacturer.contains(keyword) {
            manufacturer.print_description();  // Print the details
            println!();                        // Add a blank line for readability
            found_any = true;                  // Mark that we found at least one match
        }
    }
    
    // Step 5: Let the user know if we didn't find anything
    if !found_any {
        println!("No manufacturers found matching '{}'", keyword);
    }

    // Return Ok(()) to indicate the program completed successfully
    Ok(())
}
