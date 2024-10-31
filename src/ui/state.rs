use crate::models::user::User;

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

pub struct AppState {
    pub current_page: Page,
    pub users: Vec<User>,
    pub selected_user_index: usize,
    pub selected_wallet_index: usize,
    pub menu_items: Vec<MenuItem>,
    pub focused_menu_items: Vec<MenuItem>,
    pub current_menu_item: usize,
    pub portfolio_data: String,
    pub account_menu_items: Vec<String>,
    pub selected_account_menu_item: usize,
    pub account_focus: AccountFocus,
    pub positions_focus: PositionsFocus,
    pub positions_menu_items: Vec<String>,
    pub selected_positions_menu_item: usize,
    pub selected_fungible_token_index: usize,
    pub selected_nft_index: usize,
    pub selected_liquidity_position_index: usize,
}

impl AppState {
    pub fn new(portfolio_data: String, user: User) -> Self {
        let menu_items = vec![
            MenuItem::new("p", "Crypto Positions", Page::Positions, ""),
            MenuItem::new("w", "Watch List", Page::WatchList, ""),
            MenuItem::new("a", "Account", Page::Account, ""),
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
            portfolio_data,
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

    pub fn current_user(&self) -> Option<&User> {
        self.users.get(self.selected_user_index)
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
            // Add other pages here when they have a content focus
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
}
