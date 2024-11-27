mod db;
mod models;
mod services;
mod ui;
mod utils;

use utils::spinner::Spinner;
use utils::ascii_art::render_landing_page;

use db::Database;
use services::price::{fetch_ada_price, fetch_btc_price};
use services::user_service::UserService;
use ui::{App, run_app};

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Database::load().unwrap_or_else(|_| {
        let db = Database::new();
        db.save().expect("Failed to save new database");
        db
    });

    let user_service = UserService::new(database);
    let user_id = prompt_for_user_name(user_service.get_database()).await;
    user_service.login(&user_id).await?;
    let user = user_service.get_current_user().await
        .expect("User should be logged in")
        .clone();

    render_landing_page();
    let (portfolio_data, ada_price, btc_price) = Spinner::spin_while(
        "Loading portfolio data...",
        async {
            let portfolio = user_service.fetch_portfolio_data().await?;
            let ada_price = fetch_ada_price().await?;
            let btc_price = fetch_btc_price().await?;
            Ok::<_, Box<dyn std::error::Error>>((portfolio, ada_price, btc_price))
        }
    ).await?;

    let mut app = App::new(portfolio_data, user, user_service, ada_price, btc_price);
    run_app(&mut app).await?;

    Ok(())
}

async fn prompt_for_user_name(database: Arc<Mutex<Database>>) -> String {
    println!("Enter your name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    let db = database.lock().await;
    if let Some(user) = db.get_user_by_name(&name) {
        user.id.clone()
    } else {
        println!("User not found. Creating a new user.");
        let api_key = prompt_for_api_key();
        drop(db);
        let wallet_info = prompt_for_wallet();
        
        let mut db = database.lock().await;
        let user_id = db.create_user(name, api_key);
        db.add_wallet(&user_id, wallet_info.0, vec![wallet_info.1])
            .expect("Failed to add wallet");
        db.save().expect("Failed to save database");
        user_id
    }
}

fn prompt_for_api_key() -> String {
    println!("Enter your Taptools API key:");
    let mut api_key = String::new();
    std::io::stdin().read_line(&mut api_key).unwrap();
    api_key.trim().to_string()
}

fn prompt_for_wallet() -> (String, String) {
    println!("Enter your wallet name:");
    let mut wallet_name = String::new();
    std::io::stdin().read_line(&mut wallet_name).unwrap();
    let wallet_name = wallet_name.trim().to_string();

    println!("Enter your wallet address:");
    let mut wallet_address = String::new();
    std::io::stdin().read_line(&mut wallet_address).unwrap();
    let wallet_address = wallet_address.trim().to_string();

    (wallet_name, wallet_address)
}
