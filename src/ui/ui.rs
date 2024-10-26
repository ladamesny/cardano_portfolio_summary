use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState},
    Frame,
};

use super::state::{AppState, Page};

pub fn draw<B: Backend>(f: &mut Frame, state: &AppState) {
    let chunks = create_main_layout(f.area());
    
    draw_navigation(f, state, chunks.navigation);
    draw_page_title(f, state, chunks.title);
    
    match state.current_page() {
        Page::Account => draw_account_page(f, state, chunks.content),
        _ => draw_default_page(f, state, chunks.content),
    }
}

struct LayoutChunks {
    navigation: Rect,
    title: Rect,
    content: Rect,
}

fn create_main_layout(area: Rect) -> LayoutChunks {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(1),     // Content
            Constraint::Length(3),  // Navigation
        ].as_ref())
        .split(area);

    LayoutChunks {
        title: chunks[0],
        content: chunks[1],
        navigation: chunks[2],
    }
}

fn draw_navigation(f: &mut Frame, state: &AppState, area: Rect) {
    let menu: Vec<String> = state.menu_items.iter().enumerate().map(|(index, item)| {
        if index == state.current_menu_item {
            format!("[{}] {}", item.key, item.label)
        } else {
            format!("({}) {}", item.key, item.label)
        }
    }).collect();
    let menu_text = menu.join(" | ");
    let menu_paragraph = Paragraph::new(menu_text)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(menu_paragraph, area);
}

fn draw_page_title(f: &mut Frame, state: &AppState, area: Rect) {
    let title = &state.menu_items[state.current_menu_item].label;
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let title_paragraph = Paragraph::new(title.clone())
        .block(title_block)
        .style(Style::default().fg(Color::White));
    f.render_widget(title_paragraph, area);
}

fn draw_default_page(f: &mut Frame, state: &AppState, area: Rect) {
    let current_item = &state.menu_items[state.current_menu_item];
    let content_block = Block::default().borders(Borders::ALL);
    let content_paragraph = Paragraph::new(current_item.content.clone())
        .block(content_block)
        .style(Style::default().fg(Color::White));
    f.render_widget(content_paragraph, area);
}

pub fn draw_account_page(f: &mut Frame, state: &AppState, area: Rect) {
    // This area is now just the content area, between title and navigation
    let account_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(area);

    let left_menu = Block::default()
        .borders(Borders::ALL)
        .title("Account Menu");

    let items: Vec<ListItem> = state.account_menu_items
        .iter()
        .map(|item| ListItem::new(item.clone()))
        .collect();

    let list = List::new(items)
        .block(left_menu)
        .highlight_style(Style::default().bg(Color::Rgb(50, 0, 50)).fg(Color::White))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_account_menu_item));

    f.render_stateful_widget(list, account_chunks[0], &mut list_state);

    let selected_item = &state.account_menu_items[state.selected_account_menu_item];
    let right_content = Block::default()
        .borders(Borders::ALL)
        .title(selected_item.as_str());
    f.render_widget(right_content, account_chunks[1]);
}
