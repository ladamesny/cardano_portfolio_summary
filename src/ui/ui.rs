use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Margin},
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
    // Add border to the navigation area
    let block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(block, area);

    // Calculate the inner area for content (accounting for borders)
    let content_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    // Add padding to left and right
    let padding = 2;
    let content_area = Rect {
        x: content_area.x + padding,
        width: content_area.width.saturating_sub(padding * 2),
        ..content_area
    };

    let mut menu: Vec<String> = state.menu_items.iter().enumerate()
        .filter(|(_, item)| item.key != 'q')  // Exclude 'q' from the main menu
        .map(|(index, item)| {
            if index == state.current_menu_item {
                format!("  {}  ", item.label)
            } else {
                format!("({}) {}", item.key, item.label)
            }
        }).collect();

    let menu_text = menu.join(" | ");
    
    // Find the quit item
    let quit_item = state.menu_items.iter()
        .find(|item| item.key == 'q')
        .map(|item| format!("({}) {}", item.key, item.label))
        .unwrap_or_default();

    // Calculate the space between menu items and quit
    let available_width = content_area.width as usize;
    let menu_width = menu_text.len();
    let quit_width = quit_item.len();
    let spacing = available_width.saturating_sub(menu_width + quit_width);

    // Combine menu items, spacing, and quit item
    let full_menu_text = format!("{}{:spacing$}{}", menu_text, "", quit_item, spacing = spacing);

    let menu_paragraph = Paragraph::new(full_menu_text)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(menu_paragraph, content_area);

    // Highlight the selected item
    if let Some(selected_item) = state.menu_items.get(state.current_menu_item) {
        if selected_item.key != 'q' {
            let mut start_x = content_area.x;
            for (index, item) in state.menu_items.iter().enumerate() {
                if index == state.current_menu_item {
                    break;
                }
                if item.key != 'q' {
                    start_x += (item.label.len() + 7) as u16; // +7 for "() ", "  ", and " | "
                }
            }
            let width = selected_item.label.len() as u16 + 4; // +4 for padding
            let highlight_area = Rect {
                x: start_x,
                y: content_area.y,
                width,
                height: 1,
            };
            let highlight = Paragraph::new(format!("  {}  ", selected_item.label))
                .style(Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White));
            f.render_widget(highlight, highlight_area);
        }
    }
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
        .highlight_style(Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White))
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
