use ratatui::layout::{Layout, Direction, Constraint, Rect};

pub struct LayoutChunks {
    pub title: Rect,
    pub content: Rect,
    pub navigation: Rect,
}

pub fn create_main_layout(area: Rect) -> LayoutChunks {
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