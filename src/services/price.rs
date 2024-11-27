use serde_json::Value;

pub async fn fetch_ada_price() -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=cardano&vs_currencies=usd&include_24h_change=true";
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("API request failed: {} - {}", response.status(), response.text().await?).into());
    }

    let text = response.text().await?;

    let json: Value = serde_json::from_str(&text)?;
    match json.get("cardano").and_then(|c| c.get("usd")).and_then(|p| p.as_f64()) {
        Some(price) => {
            Ok(price)
        },
        None => {
            println!("Failed to parse price from JSON"); // Debug print
            Err("Failed to parse ADA price from response".into())
        }
    }
} 

pub async fn fetch_btc_price() -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_24h_change=true";
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("API request failed: {} - {}", response.status(), response.text().await?).into());
    }

    let text = response.text().await?;

    let json: Value = serde_json::from_str(&text)?;
    match json.get("bitcoin").and_then(|c| c.get("usd")).and_then(|p| p.as_f64()) {
        Some(price) => {
            Ok(price)
        },
        None => {
            println!("Failed to parse price from JSON"); // Debug print
            Err("Failed to parse BTC price from response".into())
        }
    }
}