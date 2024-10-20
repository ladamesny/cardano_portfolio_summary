mod models;
mod services;
mod ui;

use services::portfolio_api::PortfolioApiConfig;
use services::user_config::UserConfig;
use ui::{AppState, run_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_config = UserConfig::load().unwrap_or_else(|| UserConfig::prompt_for_user_config());
    let taptools_config = PortfolioApiConfig::load().unwrap_or_else(|| PortfolioApiConfig::prompt_user_for_portfolio_api_config());

    match taptools_config.get_portfolio_data(&user_config.cardano_address).await {
        Ok(data) => {
            let mut app = AppState::new(data.to_string());
            let mut terminal = ui::setup_terminal()?;
            run_app(&mut terminal, &mut app)?;
            ui::restore_terminal(&mut terminal)?;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}
