//! NHTSA vehicle-manufacturer lookup.
//!
//! Fetches the public NHTSA "get all manufacturers" endpoint and prints every
//! manufacturer whose name, common name, or country contains the user-supplied
//! keyword. The HTTP fetch and filtering loop live here in [`run`]; the
//! binary entry point in `main.rs` only parses argv.

use anyhow::{Context, Result, anyhow};
use serde_json::Value;

/// NHTSA endpoint that returns every registered manufacturer as JSON.
pub const API_URL: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";

/// View into a single manufacturer record borrowed from the parsed JSON.
pub struct Manufacturer<'a> {
    /// Official manufacturer name (`Mfr_Name`).
    pub name: Option<&'a str>,
    /// Common manufacturer name (`Mfr_CommonName`).
    pub common_name: Option<&'a str>,
    /// Country of origin (`Country`).
    pub country: Option<&'a str>,
}

/// Case-sensitive substring match across one or more fields.
pub trait Contains {
    /// Returns `true` if `keyword` appears in any searchable field.
    fn contains(&self, keyword: &str) -> bool;
}

impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, keyword: &str) -> bool {
        self.name.unwrap_or_default().contains(keyword)
            || self.common_name.unwrap_or_default().contains(keyword)
            || self.country.unwrap_or_default().contains(keyword)
    }
}

impl<'a> Manufacturer<'a> {
    /// Print all fields, one per line, in a stable human-readable format.
    pub fn print_description(&self) {
        println!("Manufacturer: {}", self.name.unwrap_or_default());
        println!("Common Name: {}", self.common_name.unwrap_or_default());
        println!("Country: {}", self.country.unwrap_or_default());
    }
}

/// Fetch the manufacturers feed and print every record matching `keyword`.
pub async fn run(keyword: &str) -> Result<()> {
    println!("Keyword: {keyword}");

    let response: Value = reqwest::get(API_URL)
        .await
        .context("HTTP request to NHTSA API failed")?
        .json::<Value>()
        .await
        .context("decoding NHTSA response as JSON")?;

    let results = response
        .get("Results")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("unexpected API shape: missing `Results` array"))?;

    let mut found_any = false;
    for item in results {
        let obj = item
            .as_object()
            .ok_or_else(|| anyhow!("unexpected API shape: manufacturer entry is not an object"))?;
        let manufacturer = Manufacturer {
            name: obj.get("Mfr_Name").and_then(Value::as_str),
            common_name: obj.get("Mfr_CommonName").and_then(Value::as_str),
            country: obj.get("Country").and_then(Value::as_str),
        };
        if manufacturer.contains(keyword) {
            manufacturer.print_description();
            println!();
            found_any = true;
        }
    }

    if !found_any {
        println!("No manufacturers found matching '{keyword}'");
    }
    Ok(())
}
