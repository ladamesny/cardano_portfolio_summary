mod app;
mod state;
mod event;
mod ui;

pub use app::{App};
pub use state::{AppState, Page};
pub use ui::{draw, setup_terminal, restore_terminal};

use std::io;
use crossterm::event::{Event, KeyCode, read};
use ratatui::Terminal;
use ratatui::backend::Backend;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw::<B>(f, app))?;

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('w') => app.set_current_page(Page::WatchList),
                KeyCode::Char('n') => app.set_current_page(Page::TopNftPositions),
                KeyCode::Char('a') => app.set_current_page(Page::Account),
                KeyCode::Enter => {
                    if app.menu_items[app.current_menu_item].page == Page::Account {
                        app.toggle_account_expanded();
                    } else {
                        app.set_current_page(app.menu_items[app.current_menu_item].page.clone());
                    }
                }
                _ => {}
            }
        }
    }
}
