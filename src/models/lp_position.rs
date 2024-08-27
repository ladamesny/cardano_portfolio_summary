use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LpPosition {
    #[serde(rename = "amountLP")]
    amount_lp: u64,
    #[serde(rename = "adaValue")]
    ada_value: f64,
    #[serde(rename = "liquidValue")]
    liquid_value: f64,
    ticker: String,
    exchange: String,
    unit: String,
    #[serde(rename = "tokenA")]
    token_a: String,
    #[serde(rename = "tokenAAmount")]
    token_a_amount: f64,
    #[serde(rename = "tokenAName")]
    token_a_name: String,
    #[serde(rename = "tokenB")]
    token_b: String,
    #[serde(rename = "tokenBAmount")]
    token_b_amount: f64,
    #[serde(rename = "tokenBName")]
    token_b_name: String,
}