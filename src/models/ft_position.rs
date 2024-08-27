use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FtPosition {
    balance: f64,
    #[serde(rename = "liquidBalance")]
    liquid_balance: f64,
    #[serde(rename = "adaValue")]
    ada_value: f64,
    #[serde(rename = "liquidValue")]
    liquid_value: f64,
    // price: f64,
    ticker: String,
    unit: String,
    fingerprint: String,
    #[serde(rename = "24h")]
    change_24h: Option<f64>, 
    #[serde(rename = "7d")]
    change_7d: Option<f64>, 
    #[serde(rename = "30d")]
    change_30d: Option<f64>, 
}

