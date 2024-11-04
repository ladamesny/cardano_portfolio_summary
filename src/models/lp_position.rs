use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LpPosition {
    pub amount_lp: u64,
    pub ada_value: f64,
    pub liquid_value: f64,
    pub ticker: String,
    pub exchange: String,
    pub unit: String,
    #[serde(rename = "tokenA")]
    pub token_a: String,
    #[serde(rename = "tokenAAmount")]
    pub token_a_amount: f64,
    #[serde(rename = "tokenAName")]
    pub token_a_name: String,
    #[serde(rename = "tokenB")]
    pub token_b: String,
    #[serde(rename = "tokenBAmount")]
    pub token_b_amount: f64,
    #[serde(rename = "tokenBName")]
    pub token_b_name: String,
}
