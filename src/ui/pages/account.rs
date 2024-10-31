use ratatui::{
    Frame, 
    layout::{Rect, Layout, Direction, Constraint},
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, List, ListItem, ListState},
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
        // Main "Profiles" container
        let profiles_block = Block::default()
            .title("Profiles")
            .borders(Borders::ALL)
            .border_style(right_content_style);
        
        let profiles_area = account_chunks[1];
        let inner_profiles_area = profiles_block.inner(profiles_area);

        // Split the inner area for the two columns
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(inner_profiles_area);

        // Render the main profiles container
        f.render_widget(profiles_block, profiles_area);

        // Create vertical layouts for both columns with 30% height containers
        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(content_layout[1]);

        // Left container - User Profiles
        let profile_block = Block::default()
            .title("User Profiles")
            .borders(Borders::ALL)
            .border_style(right_content_style);
        
        let inner_area = profile_block.inner(left_layout[0]);
        
        // Create list items for all users
        let user_items: Vec<ListItem> = state.users
            .iter()
            .enumerate()
            .map(|(index, user)| {
                let style = if index == state.selected_user_index && state.account_focus == AccountFocus::Content {
                    Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(user.name.clone()).style(style)
            })
            .collect();
        
        f.render_widget(profile_block, left_layout[0]);
        f.render_widget(
            List::new(user_items)
                .style(Style::default().fg(Color::White)),
            inner_area,
        );

        // Right container - Wallets
        let wallet_block = Block::default()
            .title("Wallets")
            .borders(Borders::ALL)
            .border_style(right_content_style);
        
        let inner_wallet_area = wallet_block.inner(right_layout[0]);
        
        // Get wallets for the selected user
        let wallet_items: Vec<ListItem> = if let Some(selected_user) = state.users.get(state.selected_user_index) {
            selected_user.wallets
                .iter()
                .enumerate()
                .map(|(index, w)| {
                    let style = if index == state.selected_wallet_index && state.account_focus == AccountFocus::Content {
                        Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(format!("{} ({})", w.name, w.addresses.join(", ")))
                        .style(style)
                })
                .collect()
        } else {
            Vec::new()
        };
        
        f.render_widget(wallet_block, right_layout[0]);
        f.render_widget(
            List::new(wallet_items)
                .style(Style::default().fg(Color::White)),
            inner_wallet_area,
        );
    } else {
        // Other menu items just show a simple content block
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_style(right_content_style)
            .title(selected_item.as_str());
        f.render_widget(content_block, account_chunks[1]);
    }
}
