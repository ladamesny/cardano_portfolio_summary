use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
};
use crate::ui::state::AppState;

pub fn draw_top_nft_positions_page(f: &mut Frame, state: &mut AppState, area: Rect) {
    let block = Block::default()
        .title("Top NFT Positions")
        .borders(Borders::ALL)
        .style(Style::default());
    
    let inner_area = block.inner(area);
    
    f.render_widget(block, area);
    f.render_widget(
        Paragraph::new("Top NFT Positions content coming soon...")
            .style(Style::default().fg(Color::White)),
        inner_area,
    );
}
