use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
};
use crate::ui::state::AppState;

pub fn draw_watch_list_page(f: &mut Frame, state: &mut AppState, area: Rect) {
    let block = Block::default()
        .title("Watch List")
        .borders(Borders::ALL)
        .style(Style::default());
    
    let inner_area = block.inner(area);
    
    f.render_widget(block, area);
    f.render_widget(
        Paragraph::new("Watch List content coming soon...")
            .style(Style::default().fg(Color::White)),
        inner_area,
    );
}
