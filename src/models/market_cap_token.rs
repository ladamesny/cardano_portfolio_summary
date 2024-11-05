use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketCapToken {
    #[serde(rename = "circSupply")]
    pub circ_supply: f64,
    pub fdv: f64,
    pub mcap: f64,
    pub price: f64,
    pub ticker: String,
    #[serde(rename = "totalSupply")]
    pub total_supply: f64,
    pub unit: String,
} 