#[derive(Clone, PartialEq)]
pub enum Page {
    TopNftPositions,
    WatchList,
    Account,
    Quit,
}

pub struct MenuItem {
    pub key: char,
    pub label: String,
    pub page: Page,
    pub content: String,
}

impl MenuItem {
    pub fn new(key: char, label: &str, page: Page, content: &str) -> Self {
        MenuItem {
            key,
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

pub struct AppState {
    pub menu_items: Vec<MenuItem>,
    pub current_menu_item: usize,
    pub portfolio_data: String,
    pub account_menu_items: Vec<String>,
    pub selected_account_menu_item: usize,
    pub account_focus: AccountFocus,
}

impl AppState {
    pub fn new(portfolio_data: String) -> Self {
        let menu_items = vec![
            MenuItem::new('n', "Top NFT Positions", Page::TopNftPositions, "This is the content inside the block"),
            MenuItem::new('w', "Watch List", Page::WatchList, "This is the watch List"),
            MenuItem::new('q', "Quit", Page::Quit, ""),
            MenuItem::new('a', "Account", Page::Account, ""),
        ];
        AppState {
            menu_items,
            current_menu_item: 0,
            portfolio_data,
            account_menu_items: vec!["Profile".to_string(), "Wallets".to_string()],
            selected_account_menu_item: 0,
            account_focus: AccountFocus::Menu,
        }
    }

    pub fn reset_account_focus(&mut self) {
        self.account_focus = AccountFocus::Menu;
    }

    pub fn set_current_page(&mut self, page: Page) {
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
}
