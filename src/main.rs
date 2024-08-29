mod models;
mod services;

use serde::{Deserialize, Serialize};

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::execute;
use crossterm::terminal::ClearType;
use crossterm::terminal::Clear;
use std::io::{stdout};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

use services::portfolio_api::PortfolioApiConfig;
use services::user_config::UserConfig;

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


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let user_config = if let Some(config) = UserConfig::load() {
        config
    } else {
        UserConfig::prompt_for_user_config()
    };

    let taptools_config = if let Some(config) = PortfolioApiConfig::load() {
        config
    } else {
        PortfolioApiConfig::prompt_user_for_portfolio_api_config()
    };

    match taptools_config.get_portfolio_data(&user_config.cardano_address).await {
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
