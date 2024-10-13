#[derive(Clone, PartialEq)]
pub enum Page {
    TopNftPositions,
    WatchList,
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

pub struct AppState {
    pub menu_items: Vec<MenuItem>,
    pub current_menu_item: usize,
    pub portfolio_data: String,
}

impl AppState {
    pub fn new(portfolio_data: String) -> Self {
        let menu_items = vec![
            MenuItem::new('n', "Top NFT Positions", Page::TopNftPositions, "This is the content inside the block"),
            MenuItem::new('w', "Watch List", Page::WatchList, "This is the watch List"),
            MenuItem::new('q', "Quit", Page::Quit, ""),
        ];
        AppState {
            menu_items,
            current_menu_item: 0,
            portfolio_data,
        }
    }

    pub fn next(&mut self) {
        self.current_menu_item = (self.current_menu_item + 1) % self.menu_items.len();
    }

    pub fn previous(&mut self) {
        if self.current_menu_item > 0 {
            self.current_menu_item -= 1;
        } else {
            self.current_menu_item = self.menu_items.len() - 1;
        }
    }

    pub fn set_current_page(&mut self, page: Page) {
        if let Some(index) = self.menu_items.iter().position(|item| item.page == page) {
            self.current_menu_item = index;
        }
    }
}
