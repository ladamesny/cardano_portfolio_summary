mod models;

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

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_key: String,
    cardano_address: String,
}

fn read_config(file_path: &str) -> Option<Config> {
    if let Ok(config_data) = fs::read_to_string(file_path) {
        if let Ok(config) = serde_json::from_str(&config_data) {
            return Some(config);
        }
    }
    None
}

fn write_config(file_path: &str, config: &Config) {
    let config_data = serde_json::to_string_pretty(config).unwrap();
    fs::write(file_path, config_data).unwrap();
}

fn prompt_user_for_config() -> Config {
    let mut api_key = String::new();
    let mut cardano_address = String::new();

    println!("Enter your TapTools API key:");
    io::stdin().read_line(&mut api_key).unwrap();
    println!("Enter your Cardano address:");
    io::stdin().read_line(&mut cardano_address).unwrap();

    Config {
        api_key: api_key.trim().to_string(),
        cardano_address: cardano_address.trim().to_string(),
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file_path = "config.json";

    let config = if let Some(config) = read_config(config_file_path) {
        config
    } else {
        let config = prompt_user_for_config();
        write_config(config_file_path, &config);
        config
    };


    // Construct TapTools API URL
    let positions_url = format!("https://openapi.taptools.io/api/v1/wallet/portfolio/positions?address={}", config.cardano_address);
    match get_portfolio_data(&positions_url, &config.api_key).await {
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

async fn get_portfolio_data(positions_url: &str, api_key: &str) -> Result<PortfolioSummary, reqwest::Error> {
    // Create a new client
    let client = Client::new();
    
    // Create a header map and insert the authorization header
    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(api_key).unwrap());
        
    // Make API request and handle response
    let response = client.get(positions_url)
        .headers(headers)
        .send()
        .await?;
        // Deserialize the response into a PortfolioSummary object
    let portfolio_summary = response.json::<PortfolioSummary>().await?;
    Ok(portfolio_summary)
}