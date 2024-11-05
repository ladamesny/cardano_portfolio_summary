use crate::db::Database;
use crate::models::user::User;
use crate::services::portfolio_api::PortfolioApiConfig;
use crate::models::market_cap_token::MarketCapToken;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct UserService {
    database: Arc<Mutex<Database>>,
    current_user: Arc<Mutex<Option<User>>>,
    portfolio_api: Arc<Mutex<Option<PortfolioApiConfig>>>,
}

impl UserService {
    pub fn new(database: Database) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
            current_user: Arc::new(Mutex::new(None)),
            portfolio_api: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn login(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.database.lock().await;
        if let Some(user) = db.get_user(user_id) {
            let mut current_user = self.current_user.lock().await;
            let mut portfolio_api = self.portfolio_api.lock().await;
            *portfolio_api = Some(PortfolioApiConfig::new(&user.taptools_api_key));
            *current_user = Some(user.clone());
            Ok(())
        } else {
            Err("User not found".into())
        }
    }
    pub async fn fetch_portfolio_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let current_user = self.current_user.lock().await;
        let portfolio_api = self.portfolio_api.lock().await;
        
        if let (Some(user), Some(api)) = (current_user.as_ref(), portfolio_api.as_ref()) {
            if let Some(wallet) = user.wallets.first() {
                if let Some(address) = wallet.addresses.first() {
                    return Ok(api.get_portfolio_data(address).await?);
                }
            }
            Err("No wallet or address found".into())
        } else {
            Err("Not logged in".into())
        }
    }
    pub async fn get_current_user(&self) -> Option<User> {
        self.current_user.lock().await.clone()
    }

    pub fn get_database(&self) -> Arc<Mutex<Database>> {
        self.database.clone()
    }

    pub async fn get_market_cap_data(&self) -> Result<Vec<MarketCapToken>, Box<dyn std::error::Error>> {
        let portfolio_api = self.portfolio_api.lock().await;
        
        if let Some(api) = portfolio_api.as_ref() {
            Ok(api.get_market_cap_data().await?)
        } else {
            Err("Not logged in".into())
        }
    }
}
