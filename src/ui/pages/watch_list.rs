use ratatui::{
    Frame,
    layout::{Rect, Layout, Direction, Constraint},
    style::{Style, Color, Modifier},
    prelude::Margin,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use crate::ui::state::{AppState, WatchListFocus};

pub fn draw_watch_list_page(f: &mut Frame, state: &mut AppState, area: Rect) {
    let main_block = Block::default()
        .title("Watch List")
        .borders(Borders::ALL)
        .style(Style::default());

    let inner_area = main_block.inner(area);

    // Create horizontal split for menu and content
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(inner_area);

    // Render the main block
    f.render_widget(main_block, area);

    // Left Menu
    let left_menu_style = if state.watch_list_focus == WatchListFocus::Menu {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let left_menu = Block::default()
        .borders(Borders::ALL)
        .border_style(left_menu_style)
        .title("Watch List Menu");

    let items = vec![
        ListItem::new("Recommended Trades"),
        ListItem::new("Watching"),
    ];

    let list = List::new(items)
        .block(left_menu)
        .highlight_style(Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_watch_list_menu_item));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // Right Content
    let content_titles = ["Recommended Trades", "Watching"];
    let selected_item = content_titles[state.selected_watch_list_menu_item];
    let right_content_style = if state.watch_list_focus == WatchListFocus::Content {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(right_content_style)
        .title(selected_item);

    let margin = Margin {
        horizontal: 1,
        vertical: 1,
    };
    
    f.render_widget(&content_block, chunks[1]);
    match state.selected_watch_list_menu_item {
        0 => draw_recommended_trades(f, state, chunks[1].inner(margin)),
        1 => draw_watching(f, state, chunks[1].inner(margin)),
        _ => unreachable!(),
    };
}

fn draw_recommended_trades(f: &mut Frame, state: &AppState, area: Rect) {
    // TODO: Implement recommended trades table similar to positions tables
}

fn draw_watching(f: &mut Frame, state: &AppState, area: Rect) {
    // TODO: Implement watching table similar to positions tables
}
