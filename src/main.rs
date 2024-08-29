mod models;
mod services;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::execute;
use crossterm::terminal::ClearType;
use crossterm::terminal::Clear;
use std::io::{stdout, Write};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;


use models::portfolio_summary::{PortfolioSummary};
use services::portfolio_api::{
    read_portfolio_api_config, prompt_user_for_portfolio_api_config, write_portfolio_api_config, get_portfolio_data
};

// User-Specific Configuration (separate from TapTools)
#[derive(Debug, Serialize, Deserialize)]
struct UserConfig {
    cardano_address: String,
    // Add 'watch_list' here in the future
}

#[derive(Clone,PartialEq)]
enum Page {
    TopNftPositions,
    Quit,
}

struct MenuItem {
    key: char,
    label: &'static str,
    page: Page,
}

impl MenuItem {
    fn new(key: char, label: &'static str, page: Page) -> Self {
        MenuItem { key, label, page }
    }
    
}

fn read_user_config(file_path: &str) -> Option<UserConfig> {
    if let Ok(config_data) = fs::read_to_string(file_path) {
        if let Ok(config) = serde_json::from_str(&config_data) {
            return Some(config);
        }
    }
    None
}

fn write_user_config(file_path: &str, config: &UserConfig) {
    let config_data = serde_json::to_string_pretty(config).unwrap();
    fs::write(file_path, config_data).unwrap();
}

// Prompt user for PortfolioApi API key
pub fn prompt_user_for_user_config() -> UserConfig {
    let mut cardano_address = String::new();

    println!("Enter your Cardano address:");
    std::io::stdin().read_line(&mut cardano_address).unwrap();

    UserConfig {
        cardano_address: cardano_address.trim().to_string(),
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let user_config_file_path = "user_config.json";
    let taptools_config_file_path = "taptools_config.json"; 

    let user_config = if let Some(config) = read_user_config(user_config_file_path) {
        config
    } else {
        let config = prompt_user_for_user_config();
        write_user_config(user_config_file_path, &config);
        config
    };

    let taptools_config = if let Some(config) = read_portfolio_api_config(taptools_config_file_path) {
        config
    } else {
        let config = prompt_user_for_portfolio_api_config();
        write_portfolio_api_config(taptools_config_file_path, &config);
        config
    };

    let positions_url = format!("https://openapi.taptools.io/api/v1/wallet/portfolio/positions?address={}", user_config.cardano_address);
    match get_portfolio_data(&positions_url, &taptools_config.api_key).await {
        Ok(data) => {
            // Initialize Terminal UI
            enable_raw_mode()?;
            let mut stdout = stdout();
            execute!(stdout, Clear(ClearType::All))?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            let menu_items = vec![
                MenuItem::new('n', "Top NFT Positions", Page::TopNftPositions),
                MenuItem::new('q', "Quit", Page::Quit),
                // Add more menu items here
            ];

            let mut current_page = Page::TopNftPositions;

            loop {
                terminal.draw(|f| {
                    let size = f.size();
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                        .split(size);

                    let block = Block::default().borders(Borders::ALL).title("Main Area");
                    f.render_widget(block, chunks[0]);

                    let menu: Vec<String> = menu_items.iter().map(|item| format!("({}) {}", item.key, item.label)).collect();
                    let menu_text = menu.join(" | ");
                    let menu_paragraph = Paragraph::new(menu_text).style(Style::default().fg(Color::Yellow));
                    f.render_widget(menu_paragraph, chunks[1]);
                })?;

                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            if let Some(menu_item) = menu_items.iter().find(|item| item.key == c) {
                                if menu_item.page == Page::Quit {
                                    disable_raw_mode()?;
                                    break;
                                }
                                current_page = menu_item.page.clone();
                            }
                        }
                        KeyCode::Esc => {
                            disable_raw_mode()?;
                            break;
                        }
                        _ => {}
                    }
                }
            }

            disable_raw_mode()?;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}
