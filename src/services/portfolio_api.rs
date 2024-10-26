use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};

use crate::models::portfolio_summary::PortfolioSummary;

const PORTFOLIO_API_HOST: &str = "https://openapi.taptools.io/api/v1";
const WALLET_POSITIONS_URL: &str = "/wallet/portfolio/positions";

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
    pub async fn get_portfolio_data(&self, address: &str) -> Result<PortfolioSummary, reqwest::Error> {
        let url = format!("{}{}?address={}", PORTFOLIO_API_HOST, WALLET_POSITIONS_URL, address);
        self.make_portfolio_api_request(&url).await?.json::<PortfolioSummary>().await
    }

    // Internal function to make PortfolioApi API requests
    async fn make_portfolio_api_request(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(self.api_key.as_str()).unwrap());
        client.get(url).headers(headers).send().await
    }
}
