use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};

use crate::models::portfolio_summary::PortfolioSummary;

const CONFIG_FILE_PATH: &str = "taptools_config.json";
const PORTFOLIO_API_HOST: &str = "https://openapi.taptools.io/api/v1";
const WALLET_POSITIONS_URL: &str = "/wallet/portfolio/positions";

// PortfolioApi-Specific Configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioApiConfig {
    pub api_key: String,
}

impl PortfolioApiConfig {
    // Read PortfolioApi configuration
    pub fn load() -> Option<PortfolioApiConfig> {
        if let Ok(config_data) = std::fs::read_to_string(CONFIG_FILE_PATH) {
            if let Ok(config) = serde_json::from_str(&config_data) {
                return Some(config);
            }
        }
        None
    }

    // Writ PortfolioApi configuration
    pub fn save(&self) {
        let config_data = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(CONFIG_FILE_PATH, config_data).unwrap();
    }

    // Prompt user for PortfolioApi API key
    pub fn prompt_user_for_portfolio_api_config() -> PortfolioApiConfig {
        let mut api_key = String::new();

        println!("Enter your Taptools API key:");
        std::io::stdin().read_line(&mut api_key).unwrap();

        let config = PortfolioApiConfig {
            api_key: api_key.trim().to_string(),
        };

        config.save();
        config
    }

    // Fetch portfolio data
    pub async fn get_portfolio_data(&self, address: &str) -> Result<PortfolioSummary, reqwest::Error> {
        let url = format!("{}{}?address={}", PORTFOLIO_API_HOST, WALLET_POSITIONS_URL, address);
        self.make_portfolio_api_request(&url).await?.json::<PortfolioSummary>().await
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
    async fn make_portfolio_api_request(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(self.api_key.as_str()).unwrap());
        client.get(url).headers(headers).send().await
    }
}