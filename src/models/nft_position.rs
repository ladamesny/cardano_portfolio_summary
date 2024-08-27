use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NftPosition {
    balance: u32, 
    #[serde(rename = "adaValue")]
    ada_value: f64,
    #[serde(rename = "liquidValue")]
    liquid_value: f64,
    #[serde(rename = "floorPrice")]
    floor_price: f64,
    listings: u32, 
    name: String,
    policy: String,
    #[serde(rename = "24h")]
    change_24h: Option<f64>, 
    #[serde(rename = "7d")]
    change_7d: Option<f64>, 
    #[serde(rename = "30d")]
    change_30d: Option<f64>, 
}