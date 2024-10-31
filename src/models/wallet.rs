use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub id: String,
    pub name: String,
    pub addresses: Vec<String>,
}