use crate::ui::{
    AppState, 
    draw, 
    Page,
    state::{
        PositionsFocus,
        WatchListFocus,
        AccountFocus,
    }
};
use crate::models::user::User;
use crate::services::user_service::UserService;
use crate::utils::spinner::Spinner;
use std::io;
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

use crate::services::price::{fetch_ada_price, fetch_btc_price};

pub struct App {
    pub state: AppState,
    pub user_service: UserService,
}

impl App {
    pub fn new(portfolio_data: String, user: User, user_service: UserService, ada_price: f64, btc_price: f64) -> Self {
        App {
            state: AppState::new(portfolio_data, user, ada_price, btc_price),
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
                                match self.state.current_page() {
                                    Page::Positions if self.state.positions_focus == PositionsFocus::Content => {
                                        self.state.toggle_positions_focus()
                                    },
                                    Page::WatchList if self.state.watch_list_focus == WatchListFocus::Content => {
                                        self.state.toggle_watch_list_focus()
                                    },
                                    Page::Account if self.state.account_focus == AccountFocus::Content => {
                                        self.state.toggle_account_focus()
                                    },
                                    _ => {}
                                }
                            },
                            _ => {
                                if self.state.is_content_focused() {
                                    self.handle_content_input(code).await?;
                                } else {
                                    self.handle_menu_input(code).await?;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    async fn handle_watch_list_navigation(&mut self) {
        if self.state.selected_watch_list_menu_item == 2 {  // Market Caps index
            match self.user_service.get_market_cap_data().await {
                Ok(tokens) => {
                    self.state.market_cap_tokens = tokens;
                }
                Err(e) => {
                    eprintln!("Failed to fetch market cap data: {}", e);
                }
            }
        }
    }

    async fn handle_menu_input(&mut self, code: KeyCode) -> io::Result<()> {
        match code {
            KeyCode::Down | KeyCode::Char('j') => {
                match self.state.current_page() {
                    Page::Account => self.state.next_account_menu_item(),
                    Page::Positions => self.state.next_positions_menu_item(),
                    Page::WatchList => {
                        self.state.next_watch_list_menu_item();
                        self.handle_watch_list_navigation().await;
                    }
                    _ => {}
                }
            },
            KeyCode::Up | KeyCode::Char('k') => {
                match self.state.current_page() {
                    Page::Account => self.state.previous_account_menu_item(),
                    Page::Positions => self.state.previous_positions_menu_item(),
                    Page::WatchList => self.state.previous_watch_list_menu_item(),
                    _ => {}
                }
            },
            KeyCode::Char('r') => {
                match self.state.current_page() {
                    Page::Positions => {
                        let _ = self.refresh_data().await;
                    },
                    Page::WatchList if self.state.selected_watch_list_menu_item == 2 => {
                        if let Ok(tokens) = self.user_service.get_market_cap_data().await {
                            self.state.market_cap_tokens = tokens;
                        }
                    },
                    _ => {}
                }
            },
            KeyCode::Char('w') => self.state.set_current_page(Page::WatchList),
            KeyCode::Char('p') => self.state.set_current_page(Page::Positions),
            KeyCode::Char('a') => self.state.set_current_page(Page::Account),
            KeyCode::Enter => {
                match self.state.current_page() {
                    Page::Account => self.state.toggle_account_focus(),
                    Page::Positions if self.state.positions_focus == PositionsFocus::Menu => {
                        self.state.toggle_positions_focus()
                    },
                    Page::WatchList if self.state.watch_list_focus == WatchListFocus::Menu => {
                        self.state.toggle_watch_list_focus()
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn next_market_cap_row(&mut self) {
        if let Some(selected) = self.state.selected_market_cap_row {
            if selected < self.state.market_cap_tokens.len().saturating_sub(1) {
                self.state.selected_market_cap_row = Some(selected + 1);
            }
        }
    }

    fn previous_market_cap_row(&mut self) {
        if let Some(selected) = self.state.selected_market_cap_row {
            if selected > 0 {
                self.state.selected_market_cap_row = Some(selected - 1);
            }
        }
    }

    async fn handle_content_input(&mut self, code: KeyCode) -> io::Result<()> {
        match self.state.current_page() {
            Page::Positions => {
                match self.state.selected_positions_menu_item {
                    0 => { // Fungible Tokens
                        match code {
                            KeyCode::Down | KeyCode::Char('j') => self.state.next_ft_row(),
                            KeyCode::Up | KeyCode::Char('k') => self.state.previous_ft_row(),
                            _ => {}
                        }
                    },
                    1 => { // Non-Fungible Tokens
                        match code {
                            KeyCode::Down | KeyCode::Char('j') => self.state.next_nft_row(),
                            KeyCode::Up | KeyCode::Char('k') => self.state.previous_nft_row(),
                            _ => {}
                        }
                    },
                    2 => { // Liquidity Positions
                        match code {
                            KeyCode::Down | KeyCode::Char('j') => self.state.next_lp_row(),
                            KeyCode::Up | KeyCode::Char('k') => self.state.previous_lp_row(),
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Page::WatchList => {
                match self.state.selected_watch_list_menu_item {
                    2 => { // Market Caps
                        match code {
                            KeyCode::Down | KeyCode::Char('j') => self.next_market_cap_row(),
                            KeyCode::Up | KeyCode::Char('k') => self.previous_market_cap_row(),
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Page::Account => {
                // Handle account content input
            },
            _ => {}
        }
        Ok(())
    }

    pub async fn refresh_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (portfolio, ada_price, btc_price) = Spinner::spin_while(
            "Refreshing data...",
            async {
                let portfolio = self.user_service.fetch_portfolio_data().await?;
                let ada_price = fetch_ada_price().await?;
                let btc_price = fetch_btc_price().await?;
                Ok::<_, Box<dyn std::error::Error>>((portfolio, ada_price, btc_price))
            }
        ).await?;

        self.state.update_portfolio(portfolio);
        self.state.ada_usd_price = ada_price;
        self.state.btc_usd_price = btc_price;
        Ok(())
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
