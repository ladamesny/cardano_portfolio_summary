use ratatui::Frame;
use crate::ui::state::AppState;
use crate::ui::pages::{Page, account, top_nft_positions, watch_list};

pub fn draw(f: &mut Frame, state: &mut AppState) {
    let size = f.size();

    match state.current_page {
        Page::Account => account::draw_account_page(f, state, size),
        Page::TopNftPositions => top_nft_positions::draw_top_nft_positions_page(f, state, size),
        Page::WatchList => watch_list::draw_watch_list_page(f, state, size),
    }
}

