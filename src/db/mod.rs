use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;
use crate::models::{user::User, wallet::Wallet};

const DB_FILE_PATH: &str = "database.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    users: HashMap<String, User>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let db_content = fs::read_to_string(DB_FILE_PATH)?;
        let db: Database = serde_json::from_str(&db_content)?;
        Ok(db)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db_content = serde_json::to_string_pretty(self)?;
        fs::write(DB_FILE_PATH, db_content)?;
        Ok(())
    }

    pub fn create_user(&mut self, name: String, taptools_api_key: String) -> String {
        let id = Uuid::new_v4().to_string();
        let user = User {
            id: id.clone(),
            name,
            taptools_api_key,
            wallets: Vec::new(),
        };
        self.users.insert(id.clone(), user);
        id
    }

    pub fn get_user_by_name(&self, name: &str) -> Option<&User> {
        self.users.values().find(|user| user.name == name)
    }

    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub fn update_user(&mut self, id: &str, taptools_api_key: String) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(id) {
            user.taptools_api_key = taptools_api_key;
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    pub fn delete_user(&mut self, id: &str) -> Result<(), String> {
        if self.users.remove(id).is_some() {
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    pub fn add_wallet(&mut self, user_id: &str, name: String, addresses: Vec<String>) -> Result<String, String> {
        if let Some(user) = self.users.get_mut(user_id) {
            let wallet_id = Uuid::new_v4().to_string();
            let wallet = Wallet {
                id: wallet_id.clone(),
                name,
                addresses,
            };
            user.wallets.push(wallet);
            Ok(wallet_id)
        } else {
            Err("User not found".to_string())
        }
    }

    // Add more methods for wallet operations as needed
}
