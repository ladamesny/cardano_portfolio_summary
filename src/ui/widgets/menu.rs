use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Color},
    text::Span,
    layout::Rect,
    Frame,
};

pub struct Menu {
    title: String,
    items: Vec<String>,
}

impl Menu {
    pub fn new(title: &str, items: Vec<String>) -> Self {
        Menu {
            title: title.to_string(),
            items,
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, state: &mut ListState) {
        let items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                ListItem::new(Span::raw(item.clone()))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(self.title.clone()))
            .highlight_style(Style::default().bg(Color::Rgb(128, 0, 128)).fg(Color::White));

        f.render_stateful_widget(list, area, state);
    }
}
