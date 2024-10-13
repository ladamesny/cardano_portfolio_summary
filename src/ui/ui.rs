use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use super::state::AppState;

pub fn draw<B: Backend>(f: &mut Frame, state: &AppState) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(size);

    let current_item = &state.menu_items[state.current_menu_item];
    let block = Block::default().borders(Borders::ALL).title(current_item.label.as_ref());
    let paragraph = Paragraph::new(current_item.content.clone())
        .block(block)
        .style(Style::default().fg(Color::White));
    f.render_widget(paragraph, chunks[0]);

    let menu: Vec<String> = state.menu_items.iter().map(|item| format!("({}) {}", item.key, item.label)).collect();
    let menu_text = menu.join(" | ");
    let menu_paragraph = Paragraph::new(menu_text).style(Style::default().fg(Color::Yellow));
    f.render_widget(menu_paragraph, chunks[1]);
}
