use ratatui::{
    Frame,
    layout::{Rect, Margin},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::{
    state::{AppState, Page},
    pages::{account, positions, watch_list, common::create_main_layout},
};

pub fn draw(f: &mut Frame, state: &mut AppState) {
    let chunks = create_main_layout(f.area());
    
    draw_navigation(f, state, chunks.navigation);
    draw_page_title(f, state, chunks.title);

    // Draw the current page in the main content area
    match state.current_page {
        Page::Account => account::draw_account_page(f, state, chunks.content),
        Page::Positions => positions::draw_positions_page(f, state, chunks.content),
        Page::WatchList => watch_list::draw_watch_list_page(f, state, chunks.content),
        _ => {},
    }
}

fn draw_navigation(f: &mut Frame, state: &AppState, area: Rect) {
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, area);

    let content_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let padding = 2;
    let content_area = Rect {
        x: content_area.x + padding,
        width: content_area.width.saturating_sub(padding * 2),
        ..content_area
    };

    let menu_items = if state.is_content_focused() {
        &state.focused_menu_items
    } else {
        &state.menu_items
    };

    let menu: Vec<String> = menu_items.iter().enumerate()
        .filter(|(_, item)| item.key != "q" && item.key != "r")
        .map(|(index, item)| {
            if index == state.current_menu_item && !state.is_content_focused() {
                format!("  {}  ", item.label)
            } else {
                format!("({}) {}", item.key, item.label)
            }
        }).collect();

    let menu_text = menu.join(" | ");
    
    let quit_item = menu_items.iter()
        .find(|item| item.key == "q")
        .map(|item| format!("({}) {}", item.key, item.label))
        .unwrap_or_default();
    let refresh_item = menu_items.iter()
        .find(|item| item.key == "r")
        .map(|item| format!("({}) {}", item.key, item.label))
        .unwrap_or_default();

    let available_width = content_area.width as usize;
    let menu_width = menu_text.len();
    let quit_width = quit_item.len();
    let refresh_width = refresh_item.len();
    let spacing = available_width.saturating_sub(menu_width + quit_width + refresh_width);

    let full_menu_text = format!("{}{:spacing$}{}{}{}", menu_text, "", refresh_item, " ", quit_item, spacing = spacing);

    let menu_paragraph = Paragraph::new(full_menu_text)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(menu_paragraph, content_area);

    if !state.is_content_focused() {
        if let Some(selected_item) = menu_items.get(state.current_menu_item) {
            if selected_item.key != "q" && selected_item.key != "r" {
                let mut start_x = content_area.x;
                for (index, item) in menu_items.iter().enumerate() {
                    if index == state.current_menu_item {
                        break;
                    }
                    if item.key != "q" && item.key != "r" {
                        start_x += (item.label.len() + 7) as u16;
                    }
                }
                let width = selected_item.label.len() as u16 + 4;
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

