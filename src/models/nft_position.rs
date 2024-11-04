use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NftPosition {
    pub balance: u32, 
    #[serde(rename = "adaValue")]
    pub ada_value: f64,
    #[serde(rename = "liquidValue")]
    pub liquid_value: f64,
    #[serde(rename = "floorPrice")]
    pub floor_price: f64,
    pub listings: u32, 
    pub name: String,
    pub policy: String,
    #[serde(rename = "24h")]
    pub change_24h: Option<f64>, 
    #[serde(rename = "7d")]
    pub change_7d: Option<f64>, 
    #[serde(rename = "30d")]
    pub change_30d: Option<f64>, 
}
