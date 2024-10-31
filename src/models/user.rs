use serde::{Serialize, Deserialize};
use crate::models::wallet::Wallet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub taptools_api_key: String,
    pub wallets: Vec<Wallet>,
}