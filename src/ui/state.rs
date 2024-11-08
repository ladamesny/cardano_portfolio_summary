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
    pub content: String,
}

impl MenuItem {
    pub fn new(key: &str, label: &str, page: Page, content: &str) -> Self {
        MenuItem {
            key: key.to_string(),
            label: label.to_string(),
            page,
            content: content.to_string(),
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
    pub selected_fungible_token_index: usize,
    pub selected_nft_index: usize,
    pub selected_liquidity_position_index: usize,
    pub watch_list_focus: WatchListFocus,
    pub selected_watch_list_menu_item: usize,
    pub market_cap_tokens: Vec<MarketCapToken>,
}

impl AppState {
    pub fn new(portfolio_json: String, user: User) -> Self {
        let portfolio: PortfolioSummary = serde_json::from_str(&portfolio_json)
            .expect("Failed to parse portfolio data");

        let menu_items = vec![
            MenuItem::new("p", "Crypto Positions", Page::Positions, ""),
            MenuItem::new("w", "Watch List", Page::WatchList, ""),
            MenuItem::new("a", "Account", Page::Account, ""),
            MenuItem::new("r", "Refresh", Page::Positions, ""),
            MenuItem::new("q", "Quit", Page::Quit, ""),
        ];

        let focused_menu_items = vec![
            MenuItem::new("q", "Quit", Page::Quit, ""),
            MenuItem::new("esc", "Back", Page::Back, ""),
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
            selected_fungible_token_index: 0,
            selected_nft_index: 0,
            selected_liquidity_position_index: 0,
            watch_list_focus: WatchListFocus::Menu,
            selected_watch_list_menu_item: 0,
            market_cap_tokens: Vec::new(),
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
        self.selected_account_menu_item = (self.selected_account_menu_item + 1) % self.account_menu_items.len();
    }

    pub fn previous_account_menu_item(&mut self) {
        if self.selected_account_menu_item == 0 {
            self.selected_account_menu_item = self.account_menu_items.len() - 1;
        } else {
            self.selected_account_menu_item -= 1;
        }
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
        self.selected_positions_menu_item = (self.selected_positions_menu_item + 1) % self.positions_menu_items.len();
    }

    pub fn previous_positions_menu_item(&mut self) {
        if self.selected_positions_menu_item == 0 {
            self.selected_positions_menu_item = self.positions_menu_items.len() - 1;
        } else {
            self.selected_positions_menu_item -= 1;
        }
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
        self.selected_watch_list_menu_item = (self.selected_watch_list_menu_item + 1) % 3;
    }

    pub fn previous_watch_list_menu_item(&mut self) {
        if self.selected_watch_list_menu_item > 0 {
            self.selected_watch_list_menu_item -= 1;
        } else {
            self.selected_watch_list_menu_item = 2;
        }
    }

    pub fn toggle_watch_list_focus(&mut self) {
        self.watch_list_focus = match self.watch_list_focus {
            WatchListFocus::Menu => WatchListFocus::Content,
            WatchListFocus::Content => WatchListFocus::Menu,
        };
    }
}
