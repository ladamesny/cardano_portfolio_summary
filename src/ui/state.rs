use crate::models::user::User;
use crate::models::{
    ft_position::FtPosition,
    nft_position::NftPosition,
    lp_position::LpPosition,
    portfolio_summary::PortfolioSummary,
};
use crate::models::market_cap_token::MarketCapToken;

#[derive(Clone, PartialEq)]
pub enum Page {
    Positions,
    WatchList,
    Account,
    Quit,
    Back,
}

pub struct MenuItem {
    pub key: String,
    pub label: String,
    pub page: Page,
}

impl MenuItem {
    pub fn new(key: &str, label: &str, page: Page) -> Self {
        MenuItem {
            key: key.to_string(),
            label: label.to_string(),
            page,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccountFocus {
    Menu,
    Content,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionsFocus {
    Menu,
    Content,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WatchListFocus {
    Menu,
    Content,
}

pub struct AppState {
    pub current_page: Page,
    pub users: Vec<User>,
    pub selected_user_index: usize,
    pub selected_wallet_index: usize,
    pub menu_items: Vec<MenuItem>,
    pub focused_menu_items: Vec<MenuItem>,
    pub current_menu_item: usize,
    pub positions_ft: Vec<FtPosition>,
    pub positions_nft: Vec<NftPosition>,
    pub positions_lp: Vec<LpPosition>,
    pub ada_balance: f64,
    pub ada_value: f64,
    pub liquid_value: f64,
    pub account_menu_items: Vec<String>,
    pub selected_account_menu_item: usize,
    pub account_focus: AccountFocus,
    pub positions_focus: PositionsFocus,
    pub positions_menu_items: Vec<String>,
    pub selected_positions_menu_item: usize,
    pub watch_list_focus: WatchListFocus,
    pub selected_watch_list_menu_item: usize,
    pub market_cap_tokens: Vec<MarketCapToken>,
    pub selected_ft_row: usize,
    pub selected_nft_row: usize,
    pub selected_lp_row: usize,
    pub ada_usd_price: f64,
}

trait CircularNavigation {
    fn next_index(&self, current: usize) -> usize;
    fn previous_index(&self, current: usize) -> usize;
}

impl<T> CircularNavigation for Vec<T> {
    fn next_index(&self, current: usize) -> usize {
        if self.is_empty() {
            0
        } else {
            (current + 1) % self.len()
        }
    }

    fn previous_index(&self, current: usize) -> usize {
        if self.is_empty() {
            0
        } else if current == 0 {
            self.len() - 1
        } else {
            current - 1
        }
    }
}

impl AppState {
    pub fn new(portfolio_json: String, user: User, ada_price: f64) -> Self {
        let portfolio: PortfolioSummary = serde_json::from_str(&portfolio_json)
            .expect("Failed to parse portfolio data");

        let menu_items = vec![
            MenuItem::new("p", "Crypto Positions", Page::Positions),
            MenuItem::new("w", "Watch List", Page::WatchList),
            MenuItem::new("a", "Account", Page::Account),
            MenuItem::new("r", "Refresh", Page::Positions),
            MenuItem::new("q", "Quit", Page::Quit),
        ];

        let focused_menu_items = vec![
            MenuItem::new("q", "Quit", Page::Quit),
            MenuItem::new("esc", "Back", Page::Back),
        ];

        AppState {
            current_page: Page::Positions,
            users: vec![user],
            selected_user_index: 0,
            selected_wallet_index: 0,
            menu_items,
            focused_menu_items,
            current_menu_item: 0,
            positions_ft: portfolio.positions_ft,
            positions_nft: portfolio.positions_nft,
            positions_lp: portfolio.positions_lp,
            ada_balance: portfolio.ada_balance,
            ada_value: portfolio.ada_value,
            liquid_value: portfolio.liquid_value,
            account_menu_items: vec![
                "Profile".to_string(),
                "Wallets".to_string(),
                "Settings".to_string(),
            ],
            selected_account_menu_item: 0,
            account_focus: AccountFocus::Menu,
            positions_focus: PositionsFocus::Menu,
            positions_menu_items: vec![
                "Fungible Tokens".to_string(),
                "Non-Fungible Tokens".to_string(),
                "Liquidity Positions".to_string(),
            ],
            selected_positions_menu_item: 0,
            watch_list_focus: WatchListFocus::Menu,
            selected_watch_list_menu_item: 0,
            market_cap_tokens: Vec::new(),
            selected_ft_row: 0,
            selected_nft_row: 0,
            selected_lp_row: 0,
            ada_usd_price: ada_price,
        }
    }

    pub fn reset_account_focus(&mut self) {
        self.account_focus = AccountFocus::Menu;
    }

    pub fn set_current_page(&mut self, page: Page) {
        self.current_page = page.clone();
        if let Some(index) = self.menu_items.iter().position(|item| item.page == page) {
            self.current_menu_item = index;
            if page != Page::Account {
                self.reset_account_focus();
            }
        }
    }

    pub fn current_page(&self) -> &Page {
        &self.menu_items[self.current_menu_item].page
    }

    pub fn next_account_menu_item(&mut self) {
        self.selected_account_menu_item = self.navigate_next(&self.account_menu_items, self.selected_account_menu_item);
    }

    pub fn previous_account_menu_item(&mut self) {
        self.selected_account_menu_item = self.navigate_previous(&self.account_menu_items, self.selected_account_menu_item);
    }

    pub fn toggle_account_focus(&mut self) {
        self.account_focus = match self.account_focus {
            AccountFocus::Menu => AccountFocus::Content,
            AccountFocus::Content => AccountFocus::Menu,
        };
    }

    pub fn is_content_focused(&self) -> bool {
        match self.current_page() {
            Page::Account => self.account_focus == AccountFocus::Content,
            Page::Positions => self.positions_focus == PositionsFocus::Content,
            Page::WatchList => self.watch_list_focus == WatchListFocus::Content,
            _ => false,
        }
    }

    pub fn next_positions_menu_item(&mut self) {
        self.selected_positions_menu_item = self.navigate_next(&self.positions_menu_items, self.selected_positions_menu_item);
    }

    pub fn previous_positions_menu_item(&mut self) {
        self.selected_positions_menu_item = self.navigate_previous(&self.positions_menu_items, self.selected_positions_menu_item);
    }

    pub fn toggle_positions_focus(&mut self) {
        self.positions_focus = match self.positions_focus {
            PositionsFocus::Menu => PositionsFocus::Content,
            PositionsFocus::Content => PositionsFocus::Menu,
        };
    }

    pub fn update_portfolio(&mut self, portfolio_json: String) {
        if let Ok(portfolio) = serde_json::from_str::<PortfolioSummary>(&portfolio_json) {
            self.positions_ft = portfolio.positions_ft;
            self.positions_nft = portfolio.positions_nft;
            self.positions_lp = portfolio.positions_lp;
            self.ada_balance = portfolio.ada_balance;
            self.ada_value = portfolio.ada_value;
            self.liquid_value = portfolio.liquid_value;
        }
    }

    pub fn next_watch_list_menu_item(&mut self) {
        self.selected_watch_list_menu_item = self.navigate_next(&self.positions_menu_items, self.selected_watch_list_menu_item);
    }

    pub fn previous_watch_list_menu_item(&mut self) {
        self.selected_watch_list_menu_item = self.navigate_previous(&self.positions_menu_items, self.selected_watch_list_menu_item);
    }

    pub fn toggle_watch_list_focus(&mut self) {
        self.watch_list_focus = match self.watch_list_focus {
            WatchListFocus::Menu => WatchListFocus::Content,
            WatchListFocus::Content => WatchListFocus::Menu,
        };
    }

    pub fn next_ft_row(&mut self) {
        self.selected_ft_row = self.navigate_next(&self.positions_ft, self.selected_ft_row);
    }

    pub fn previous_ft_row(&mut self) {
        self.selected_ft_row = self.navigate_previous(&self.positions_ft, self.selected_ft_row);
    }

    pub fn next_nft_row(&mut self) {
        self.selected_nft_row = self.navigate_next(&self.positions_nft, self.selected_nft_row);
    }

    pub fn previous_nft_row(&mut self) {
        self.selected_nft_row = self.navigate_previous(&self.positions_nft, self.selected_nft_row);
    }

    pub fn next_lp_row(&mut self) {
        self.selected_lp_row = self.navigate_next(&self.positions_lp, self.selected_lp_row);
    }

    pub fn previous_lp_row(&mut self) {
        self.selected_lp_row = self.navigate_previous(&self.positions_lp, self.selected_lp_row);
    }

    // Generic navigation methods
    fn navigate_next<T>(&self, items: &Vec<T>, current: usize) -> usize {
        items.next_index(current)
    }

    fn navigate_previous<T>(&self, items: &Vec<T>, current: usize) -> usize {
        items.previous_index(current)
    }
}
