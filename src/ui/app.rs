use crate::ui::{AppState, draw, Page};
use crate::models::user::User;
use crate::services::user_service::UserService;
use std::io;
use std::sync::Arc;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        execute,
        event::{self, Event as CrosstermEvent, KeyCode, KeyEvent},
    },
};
use std::io::stdout;
use std::time::Duration;

pub struct App {
    pub state: AppState,
    pub user_service: UserService,
}

impl App {
    pub fn new(portfolio_data: String, user: User, user_service: UserService) -> Self {
        App {
            state: AppState::new(portfolio_data, user),
            user_service,
        }
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                draw(f, &mut self.state);
            })?;

            if event::poll(Duration::from_millis(250))? {
                match event::read()? {
                    CrosstermEvent::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Esc => {
                                if self.state.is_content_focused() {
                                    self.state.toggle_account_focus();
                                }
                            },
                            _ => {
                                if self.state.is_content_focused() {
                                    // Handle content-specific actions
                                } else {
                                    match code {
                                        KeyCode::Char('r') => {
                                            if self.state.current_page() == &Page::Positions {
                                                if let Ok(new_data) = self.user_service.fetch_portfolio_data().await {
                                                    self.state.update_portfolio(new_data);
                                                }
                                            }
                                        },
                                        KeyCode::Char('w') => self.state.set_current_page(Page::WatchList),
                                        KeyCode::Char('p') => self.state.set_current_page(Page::Positions),
                                        KeyCode::Char('a') => self.state.set_current_page(Page::Account),
                                        KeyCode::Down | KeyCode::Char('j') => {
                                            if self.state.current_page() == &Page::Account {
                                                self.state.next_account_menu_item();
                                            }

                                            if self.state.current_page() == &Page::Positions {
                                                self.state.next_positions_menu_item();
                                            }

                                            if self.state.current_page() == &Page::WatchList {
                                                self.state.next_watch_list_menu_item();
                                            }
                                        },
                                        KeyCode::Up | KeyCode::Char('k') => {
                                            if self.state.current_page() == &Page::Account {
                                                self.state.previous_account_menu_item();
                                            }

                                            if self.state.current_page() == &Page::Positions {
                                                self.state.previous_positions_menu_item();
                                            }

                                            if self.state.current_page() == &Page::WatchList {
                                                self.state.previous_watch_list_menu_item();
                                            }
                                        },
                                        KeyCode::Enter => {
                                            if self.state.current_page() == &Page::Account {
                                                self.state.toggle_account_focus();
                                            }

                                            if self.state.current_page() == &Page::Positions {
                                                self.state.toggle_positions_focus();
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub async fn run_app(app: &mut App) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = app.run(&mut terminal).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
