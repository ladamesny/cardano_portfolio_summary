use ratatui::{Frame, layout::Rect, widgets::ListState};
use crate::ui::state::AppState;
use crate::ui::pages::common::create_page_layout;
use crate::ui::widgets::menu::Menu;
use crate::ui::pages::AccountFocus;

pub fn draw_account_page(f: &mut Frame, state: &mut AppState, area: Rect) {
    let chunks = create_page_layout(area);
    
    let menu_items = state.users.iter().map(|user| user.name.clone()).collect();
    let menu = Menu::new("Users", menu_items);
    
    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_user_index));
    
    menu.draw(f, chunks[0], &mut list_state);

    // Draw wallet details in chunks[1]
    if let Some(user) = state.users.get(state.selected_user_index) {
        let wallet_items = user.wallets.iter().map(|w| w.name.clone()).collect();
        let wallet_menu = Menu::new("Wallets", wallet_items);
        
        let mut wallet_state = ListState::default();
        wallet_state.select(Some(state.selected_wallet_index));
        
        wallet_menu.draw(f, chunks[1], &mut wallet_state);
    }
}

pub fn handle_input(state: &mut AppState, key: char) {
    match key {
        'j' => {
            if state.account_focus == AccountFocus::Menu {
                state.next_user();
            } else {
                state.next_wallet();
            }
        },
        'k' => {
            if state.account_focus == AccountFocus::Menu {
                state.previous_user();
            } else {
                state.previous_wallet();
            }
        },
        '\n' => state.toggle_account_focus(),
        _ => {}
    }
}
