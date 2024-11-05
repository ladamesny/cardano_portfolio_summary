use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::market_cap_token::MarketCapToken;

const PORTFOLIO_API_HOST: &str = "https://openapi.taptools.io/api/v1";
const WALLET_POSITIONS_URL: &str = "/wallet/portfolio/positions";
const MARKET_CAP_URL: &str = "/token/top/mcap";

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioApiConfig {
    pub api_key: String,
}

impl PortfolioApiConfig {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    // Fetch portfolio data
    pub async fn get_portfolio_data(&self, address: &str) -> Result<String, reqwest::Error> {
        let response = self.make_portfolio_api_request(address).await?;
        let text = response.text().await?;
        Ok(text)
    }

    // Internal function to make PortfolioApi API requests
    async fn make_portfolio_api_request(&self, address: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}?address={}", PORTFOLIO_API_HOST, WALLET_POSITIONS_URL, address);
        
        let client = Client::new();
        let response = client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send()
            .await?;
        
        Ok(response)
    }

    pub async fn get_market_cap_data(&self) -> Result<Vec<MarketCapToken>, reqwest::Error> {
        let url = format!("{}{}", PORTFOLIO_API_HOST, MARKET_CAP_URL);
        
        let client = Client::new();
        let response = client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send()
            .await?;
            
        response.json::<Vec<MarketCapToken>>().await
    }
}
