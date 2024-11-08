use ratatui::{
    Frame,
    layout::{Rect, Layout, Direction, Constraint},
    style::{Style, Color, Modifier},
    text::Span,
    prelude::Margin,
    widgets::{Block, Borders, List, ListItem, ListState, Cell, Row, Table},
};
use crate::ui::state::{AppState, PositionsFocus};
use crate::utils::formatting::{format_ada, format_number, format_change};

pub fn draw_positions_page(f: &mut Frame, state: &mut AppState, area: Rect) {
    // Format the ADA info for the title
    let ada_info = format!(
        "ADA Balance: {} | Value: {}",
        format_ada(state.ada_balance, 2),
        format_ada(state.ada_value, 2)
    );

    // Create the main block with combined title
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            format!("Positions{:>width$}", ada_info, width = area.width as usize - 10),
            Style::default().fg(Color::White)
        ));

    // Get the inner area of the main block
    let inner_area = main_block.inner(area);

    // Create the horizontal split for menu and content
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(inner_area);

    // Render the main block
    f.render_widget(main_block, area);

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
    let margin = Margin {
        horizontal: 1,
        vertical: 1,
    };
    f.render_widget(&content_block, chunks[1]);
    match state.selected_positions_menu_item {
        0 => draw_ft_positions(f, state, chunks[1].inner(margin)),
        1 => draw_nft_positions(f, state, chunks[1].inner(margin)),
        2 => draw_lp_positions(f, state, chunks[1].inner(margin)),
        _ => unreachable!(),
    };
}

pub fn draw_ft_positions(f: &mut Frame, state: &AppState, area: Rect) {
    // Define our purple color
    let highlight_color = Color::Rgb(128, 0, 128);  // Purple color
    
    // Create header cells with uppercase text and purple background
    let header_cells = ["Ticker", "Balance", "ADA Value", "Price", "24h %", "7d %", "30d %"]
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
        .height(2);  // Increased header height

    let rows = state.positions_ft.iter().enumerate().map(|(index, position)| {
        let change_24h = position.change_24h.unwrap_or(0.0) * 100.0;
        let change_7d = position.change_7d.unwrap_or(0.0) * 100.0;
        let change_30d = position.change_30d.unwrap_or(0.0) * 100.0;

        let row_style = if state.positions_focus == PositionsFocus::Content && index == state.selected_ft_row {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else if index % 2 == 0 {
            Style::default()
        } else {
            Style::default().bg(Color::Rgb(25, 0, 25)).fg(Color::White)
        };

        let row_cells = vec![
            Cell::from(position.ticker.clone()),
            Cell::from(format_number(position.balance, 2)),
            Cell::from(format_ada(position.ada_value, 2)),
            Cell::from(format_ada(position.price.unwrap_or(0.0), 4)),
            Cell::from(format_change(change_24h)),
            Cell::from(format_change(change_7d)),
            Cell::from(format_change(change_30d)),
        ];

        Row::new(row_cells)
            .style(row_style)
            .height(2)  // Increased row height for better spacing
    });

    let widths = [
        Constraint::Percentage(15),  // Ticker
        Constraint::Percentage(20),  // Balance
        Constraint::Percentage(15),  // ADA Value
        Constraint::Percentage(15),  // Price
        Constraint::Percentage(10),  // 24h
        Constraint::Percentage(10),  // 7d
        Constraint::Percentage(15),  // 30d
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default())
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}

pub fn draw_nft_positions(f: &mut Frame, state: &AppState, area: Rect) {
    let highlight_color = Color::Rgb(128, 0, 128);
    
    let header_cells = ["Name", "Balance", "Floor Price", "ADA Value", "24h %", "7d %", "30d %"]
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

    let rows = state.positions_nft.iter().enumerate().map(|(index, position)| {
        let change_24h = position.change_24h.unwrap_or(0.0) * 100.0;
        let change_7d = position.change_7d.unwrap_or(0.0) * 100.0;
        let change_30d = position.change_30d.unwrap_or(0.0) * 100.0;

        let row_style = if index % 2 == 0 {
            Style::default()
        } else {
            Style::default().bg(Color::Rgb(25, 0, 25))
        };

        let row_cells = vec![
            Cell::from(position.name.clone()),
            Cell::from(format_number(position.balance as f64, 0)),
            Cell::from(format_ada(position.floor_price, 2)),
            Cell::from(format_ada(position.ada_value, 2)),
            Cell::from(format_change(change_24h)),
            Cell::from(format_change(change_7d)),
            Cell::from(format_change(change_30d)),
        ];

        Row::new(row_cells)
            .style(row_style)
            .height(2)
    });

    let widths = [
        Constraint::Percentage(25),  // Name
        Constraint::Percentage(10),  // Balance
        Constraint::Percentage(15),  // Floor Price
        Constraint::Percentage(15),  // ADA Value
        Constraint::Percentage(10),  // 24h
        Constraint::Percentage(10),  // 7d
        Constraint::Percentage(15),  // 30d
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default())
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}

pub fn draw_lp_positions(f: &mut Frame, state: &AppState, area: Rect) {
    let highlight_color = Color::Rgb(128, 0, 128);
    
    let header_cells = ["Pool", "Token A", "Amount A", "Token B", "Amount B", "ADA Value"]
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

    let rows = state.positions_lp.iter().enumerate().map(|(index, position)| {
        let row_style = if index % 2 == 0 {
            Style::default()
        } else {
            Style::default().bg(Color::Rgb(25, 0, 25))
        };

        let row_cells = vec![
            Cell::from(format!("{} ({})", position.exchange, position.ticker)),
            Cell::from(position.token_a_name.clone()),
            Cell::from(format_number(position.token_a_amount, 2)),
            Cell::from(position.token_b_name.clone()),
            Cell::from(format_number(position.token_b_amount, 2)),
            Cell::from(format_ada(position.ada_value, 2)),
        ];

        Row::new(row_cells)
            .style(row_style)
            .height(2)
    });

    let widths = [
        Constraint::Percentage(20),  // Pool
        Constraint::Percentage(15),  // Token A
        Constraint::Percentage(15),  // Amount A
        Constraint::Percentage(15),  // Token B
        Constraint::Percentage(15),  // Amount B
        Constraint::Percentage(20),  // ADA Value
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default())
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}
