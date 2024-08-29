use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};

use crate::models::portfolio_summary::PortfolioSummary;

// PortfolioApi-Specific Configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioApiConfig {
    pub api_key: String,
}

// Read PortfolioApi configuration
pub fn read_portfolio_api_config(file_path: &str) -> Option<PortfolioApiConfig> {
    if let Ok(config_data) = std::fs::read_to_string(file_path) {
        if let Ok(config) = serde_json::from_str(&config_data) {
            return Some(config);
        }
    }
    None
}

// Writ PortfolioApi configuration
pub fn write_portfolio_api_config(file_path: &str, config: &PortfolioApiConfig) {
    let config_data = serde_json::to_string_pretty(config).unwrap();
    std::fs::write(file_path, config_data).unwrap();
}

// Prompt user for PortfolioApi API key
pub fn prompt_user_for_portfolio_api_config() -> PortfolioApiConfig {
    let mut api_key = String::new();

    println!("Enter your Taptools API key:");
    std::io::stdin().read_line(&mut api_key).unwrap();

    PortfolioApiConfig {
        api_key: api_key.trim().to_string(),
    }
}

// Fetch portfolio data
pub async fn get_portfolio_data(address: &str, api_key: &str) -> Result<PortfolioSummary, reqwest::Error> {
    let url = format!("https://openapi.taptools.io/api/v1/wallet/portfolio/positions?address={}", address);
    make_portfolio_api_request(&url, api_key).await?.json::<PortfolioSummary>().await
}

// Placeholder for future API calls
pub async fn get_market_cap_data() -> Result<(), reqwest::Error> {
    // Implementation for fetching market cap data
    todo!() 
}

pub async fn get_nft_portfolio() -> Result<(), reqwest::Error> {
    // Implementation for fetching NFT portfolio data
    todo!() 
}

// Internal function to make PortfolioApi API requests
async fn make_portfolio_api_request(url: &str, api_key: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(api_key).unwrap());
    client.get(url).headers(headers).send().await
}