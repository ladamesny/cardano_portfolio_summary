use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FtPosition {
    pub balance: f64,
    #[serde(rename = "liquidBalance")]
    pub liquid_balance: f64,
    #[serde(rename = "adaValue")]
    pub ada_value: f64,
    #[serde(rename = "liquidValue")]
    pub liquid_value: f64,
    pub price: Option<f64>,
    pub ticker: String,
    pub unit: String,
    pub fingerprint: String,
    #[serde(rename = "24h")]
    pub change_24h: Option<f64>, 
    #[serde(rename = "7d")]
    pub change_7d: Option<f64>, 
    #[serde(rename = "30d")]
    pub change_30d: Option<f64>, 
}

