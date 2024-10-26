mod db;
mod models;
mod services;
mod ui;

use db::Database;
use services::portfolio_api::PortfolioApiConfig;
use ui::{App, run_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut database = Database::load().unwrap_or_else(|_| {
        let db = Database::new();
        db.save().expect("Failed to save new database");
        db
    });

    let user_id = prompt_for_user_id(&mut database);
    let user = database.get_user(&user_id).expect("User not found");

    let portfolio_api_config = PortfolioApiConfig::new(&user.taptools_api_key);

    if user.wallets.is_empty() {
        let wallet_name = prompt_for_wallet_name();
        let address = prompt_for_cardano_address();
        database.add_wallet(&user_id, wallet_name, vec![address.clone()]).expect("Failed to add wallet");
        database.save().expect("Failed to save database");
    }

    // We need to fetch the user again after potential modifications
    let user = database.get_user(&user_id).expect("User not found");
    let address = &user.wallets[0].addresses[0]; // Using the first address of the first wallet for simplicity

    match portfolio_api_config.get_portfolio_data(address).await {
        Ok(data) => {
            let mut app = App::new(data.to_string());
            run_app(&mut app)?;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}

fn prompt_for_user_id(database: &mut Database) -> String {
    println!("Enter your user ID (leave blank to create a new user):");
    let mut user_id = String::new();
    std::io::stdin().read_line(&mut user_id).unwrap();
    let user_id = user_id.trim();

    if user_id.is_empty() {
        let api_key = prompt_for_api_key();
        database.create_user(api_key)
    } else {
        user_id.to_string()
    }
}

fn prompt_for_api_key() -> String {
    println!("Enter your Taptools API key:");
    let mut api_key = String::new();
    std::io::stdin().read_line(&mut api_key).unwrap();
    api_key.trim().to_string()
}

fn prompt_for_wallet_name() -> String {
    println!("Enter a name for your wallet:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string()
}

fn prompt_for_cardano_address() -> String {
    println!("Enter your Cardano address:");
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();
    address.trim().to_string()
}
