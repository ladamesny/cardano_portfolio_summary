use ratatui::{
    Frame, 
    layout::{Rect, Layout, Direction, Constraint},
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState},
};
use crate::ui::state::{AppState, AccountFocus};

pub fn draw_account_page(f: &mut Frame, state: &mut AppState, area: Rect) {
    let account_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(area);

    // Left Menu
    let left_menu_style = if state.account_focus == AccountFocus::Menu {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let left_menu = Block::default()
        .borders(Borders::ALL)
        .border_style(left_menu_style)
        .title("Account Menu");

    let items: Vec<ListItem> = state.account_menu_items
        .iter()
        .map(|item| ListItem::new(item.clone()))
        .collect();

    let list = List::new(items)
        .block(left_menu)
        .highlight_style(Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_account_menu_item));

    f.render_stateful_widget(list, account_chunks[0], &mut list_state);

    // Right Content
    let selected_item = &state.account_menu_items[state.selected_account_menu_item];
    let right_content_style = if state.account_focus == AccountFocus::Content {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    // Profile Section
    if state.selected_account_menu_item == 0 {  // Profile section
        if let Some(user) = state.current_user() {
            let content_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(account_chunks[1]);

            // Left container - User Profile
            let profile_block = Block::default()
                .title("User Profile")
                .borders(Borders::ALL)
                .border_style(right_content_style);
            
            let profile_area = content_layout[0];
            let inner_area = profile_block.inner(profile_area);
            
            f.render_widget(profile_block, profile_area);
            f.render_widget(
                Paragraph::new(user.name.clone())
                    .style(Style::default().fg(Color::White)),
                inner_area,
            );

            // Right container - Wallets
            let wallet_block = Block::default()
                .title("Wallets")
                .borders(Borders::ALL)
                .border_style(right_content_style);
            
            let wallet_area = content_layout[1];
            let inner_wallet_area = wallet_block.inner(wallet_area);
            
            let wallet_items: Vec<ListItem> = user.wallets
                .iter()
                .enumerate()
                .map(|(index, w)| {
                    let style = if index == state.selected_wallet_index && state.account_focus == AccountFocus::Content {
                        Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(w.name.clone()).style(style)
                })
                .collect();
            
            f.render_widget(wallet_block, wallet_area);
            f.render_widget(
                List::new(wallet_items)
                    .style(Style::default().fg(Color::White)),
                inner_wallet_area,
            );
        }
    } else {
        // Other menu items just show a simple content block
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_style(right_content_style)
            .title(selected_item.as_str());
        f.render_widget(content_block, account_chunks[1]);
    }
}

pub fn handle_input(state: &mut AppState, key: char) {
    match key {
        'j' => {
            if state.account_focus == AccountFocus::Menu {
                state.next_account_menu_item();
            } else {
                // If in content focus, move to next wallet
                if let Some(user) = state.current_user() {
                    state.selected_wallet_index = (state.selected_wallet_index + 1) % user.wallets.len();
                }
            }
        },
        'k' => {
            if state.account_focus == AccountFocus::Menu {
                state.previous_account_menu_item();
            } else {
                // If in content focus, move to previous wallet
                if let Some(user) = state.current_user() {
                    if state.selected_wallet_index == 0 {
                        state.selected_wallet_index = user.wallets.len() - 1;
                    } else {
                        state.selected_wallet_index -= 1;
                    }
                }
            }
        },
        '\n' => state.toggle_account_focus(),
        _ => {}
    }
}
