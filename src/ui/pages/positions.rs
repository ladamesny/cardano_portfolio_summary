use ratatui::{
    Frame,
    layout::{Rect, Layout, Direction, Constraint},
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use crate::ui::state::{AppState, PositionsFocus};

pub fn draw_positions_page(f: &mut Frame, state: &mut AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(area);

    // Left Menu
    let left_menu_style = if state.positions_focus == PositionsFocus::Menu {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let left_menu = Block::default()
        .borders(Borders::ALL)
        .border_style(left_menu_style)
        .title("Positions Menu");

    let items: Vec<ListItem> = state.positions_menu_items
        .iter()
        .map(|item| ListItem::new(item.clone()))
        .collect();

    let list = List::new(items)
        .block(left_menu)
        .highlight_style(Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_positions_menu_item));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // Right Content
    let selected_item = &state.positions_menu_items[state.selected_positions_menu_item];
    let right_content_style = if state.positions_focus == PositionsFocus::Content {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(right_content_style)
        .title(selected_item.as_str());
    f.render_widget(content_block, chunks[1]);
}
