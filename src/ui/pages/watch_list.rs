use ratatui::{
    Frame,
    layout::{Rect, Layout, Direction, Constraint},
    style::{Style, Color, Modifier},
    prelude::Margin,
    widgets::{Block, Borders, List, ListItem, ListState, Cell, Row, Table},
};
use crate::ui::state::{AppState, WatchListFocus};
use crate::utils::formatting::{format_ada, format_number, format_change};

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
        .title("Menu");

    let items = vec![
        ListItem::new("Recommended Trades"),
        ListItem::new("Watching"),
        ListItem::new("Market Caps"),
    ];

    let list = List::new(items)
        .block(left_menu)
        .highlight_style(Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_watch_list_menu_item));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // Right Content
    let content_titles = ["Recommended Trades", "Watching", "Market Caps"];
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
        2 => draw_market_caps(f, state, chunks[1].inner(margin)),
        _ => unreachable!(),
    };
}

fn draw_recommended_trades(f: &mut Frame, state: &AppState, area: Rect) {
    // TODO: Implement recommended trades table similar to positions tables
}

fn draw_watching(f: &mut Frame, state: &AppState, area: Rect) {
    // TODO: Implement watching table similar to positions tables
}

fn draw_market_caps(f: &mut Frame, state: &AppState, area: Rect) {
    let highlight_color = Color::Rgb(128, 0, 128);
    
    let header_cells = ["Ticker", "Price", "Market Cap", "FDV", "Circ Supply", "Total Supply"]
        .iter()
        .map(|h| {
            Cell::from(h.to_uppercase())
                .style(Style::default()
                    .bg(highlight_color)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD))
        });
    
    let header = Row::new(header_cells)
        .style(Style::default())
        .height(2);

    let rows = state.market_cap_tokens.iter().enumerate().map(|(index, token)| {
        let row_style = if index % 2 == 0 {
            Style::default()
        } else {
            Style::default().bg(Color::Rgb(25, 0, 25))
        };

        let row_cells = vec![
            Cell::from(token.ticker.clone()),
            Cell::from(format_ada(token.price, 6)),
            Cell::from(format_ada(token.mcap, 0)),
            Cell::from(format_ada(token.fdv, 0)),
            Cell::from(format_number(token.circ_supply, 0)),
            Cell::from(format_number(token.total_supply, 0)),
        ];

        Row::new(row_cells)
            .style(row_style)
            .height(2)
    });

    let widths = [
        Constraint::Percentage(15),  // Ticker
        Constraint::Percentage(15),  // Price
        Constraint::Percentage(20),  // Market Cap
        Constraint::Percentage(20),  // FDV
        Constraint::Percentage(15),  // Circ Supply
        Constraint::Percentage(15),  // Total Supply
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default())
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}
