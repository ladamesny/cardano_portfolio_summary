use ratatui::layout::{Layout, Direction, Constraint, Rect};

pub fn create_page_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area)
}

