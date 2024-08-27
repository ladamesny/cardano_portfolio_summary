mod models;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};

use models::portfolio_summary::PortfolioSummary;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_key: String,
    cardano_address: String,
}

fn read_config(file_path: &str) -> Option<Config> {
    if let Ok(config_data) = fs::read_to_string(file_path) {
        if let Ok(config) = serde_json::from_str(&config_data) {
            return Some(config);
        }
    }
    None
}

fn write_config(file_path: &str, config: &Config) {
    let config_data = serde_json::to_string_pretty(config).unwrap();
    fs::write(file_path, config_data).unwrap();
}

fn prompt_user_for_config() -> Config {
    let mut api_key = String::new();
    let mut cardano_address = String::new();

    println!("Enter your TapTools API key:");
    io::stdin().read_line(&mut api_key).unwrap();
    println!("Enter your Cardano address:");
    io::stdin().read_line(&mut cardano_address).unwrap();

    Config {
        api_key: api_key.trim().to_string(),
        cardano_address: cardano_address.trim().to_string(),
    }
}
#[tokio::main]
async fn main() {
    let config_file_path = "config.json";

    let config = if let Some(config) = read_config(config_file_path) {
        config
    } else {
        let config = prompt_user_for_config();
        write_config(config_file_path, &config);
        config
    };

    // Construct TapTools API URL
    let positions_url = format!("https://openapi.taptools.io/api/v1/wallet/portfolio/positions?address={}", config.cardano_address);
    
    // Create a new client
    let client = Client::new();
    
    // Create a header map and insert the authorization header
    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(&config.api_key).unwrap());
        
    // Make API request and handle response
    let response = client.get(&positions_url)
        .headers(headers)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        // Deserialize directly into PortfolioSummary
        let portfolio_summary: PortfolioSummary = response.json().await.unwrap(); 
        println!("Portfolio Summary: {:#?}", portfolio_summary);
    } else {
        println!("Error fetching portfolio summary. Status code: {}", response.status());
    }
}