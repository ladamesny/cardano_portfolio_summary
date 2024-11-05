mod db;
mod models;
mod services;
mod ui;
mod utils;

use db::Database;
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

    let portfolio_data = user_service.fetch_portfolio_data().await?;
    let mut app = App::new(portfolio_data, user, user_service);
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
        let mut db = database.lock().await;
        db.create_user(name, api_key)
    }
}

fn prompt_for_api_key() -> String {
    println!("Enter your Taptools API key:");
    let mut api_key = String::new();
    std::io::stdin().read_line(&mut api_key).unwrap();
    api_key.trim().to_string()
}
