use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_FILE_PATH: &str = "user_config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub cardano_address: String,
    // Add 'watch_list' here in the future
}

impl UserConfig {
    pub fn load() -> Option<UserConfig> {
        if let Ok(config_data) = fs::read_to_string(CONFIG_FILE_PATH) {
            if let Ok(config) = serde_json::from_str(&config_data) {
                return Some(config);
            }
        }
        None
    }

    pub fn save(&self) {
        let config_data = serde_json::to_string_pretty(self).unwrap();
        fs::write(CONFIG_FILE_PATH, config_data).unwrap();
    }

    // Prompt user for PortfolioApi API key
    pub fn prompt_for_user_config() -> UserConfig {
        let mut cardano_address = String::new();

        println!("Enter your Cardano address:");
        std::io::stdin().read_line(&mut cardano_address).unwrap();

        let config = UserConfig {
            cardano_address: cardano_address.trim().to_string(),
        };

        config.save();
        config
    }
}