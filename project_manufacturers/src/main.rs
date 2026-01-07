#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // reqwest example
    let response = reqwest::get("https://api.github.com").await?;
    let body = response.text().await?;
    println!("body: {}", body);
    Ok(())
}
