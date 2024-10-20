use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, List, ListItem, ListState};
use ratatui::Frame;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use std::io::{self, Stdout};

use super::state::AppState;

pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

pub fn draw<B: Backend>(f: &mut Frame, state: &AppState) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(size);

    let current_item = &state.menu_items[state.current_menu_item];
    
    match current_item.label.as_ref() {
        "Account" => draw_account_page(f, state, chunks[0]),
        _ => {
            let block = Block::default().borders(Borders::ALL).title(current_item.label.as_ref());
            let paragraph = Paragraph::new(current_item.content.clone())
                .block(block)
                .style(Style::default().fg(Color::White));
            f.render_widget(paragraph, chunks[0]);
        }
    }

    let menu: Vec<String> = state.menu_items.iter().map(|item| format!("({}) {}", item.key, item.label)).collect();
    let menu_text = menu.join(" | ");
    let menu_paragraph = Paragraph::new(menu_text).style(Style::default().fg(Color::Yellow));
    f.render_widget(menu_paragraph, chunks[1]);
}

fn draw_account_page(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(area);

    let left_menu = Block::default()
        .borders(Borders::ALL)
        .title("Menu");

    let menu_items = vec!["Profile", "Wallets"];
    let items: Vec<ListItem> = menu_items
        .iter()
        .map(|&item| ListItem::new(item))
        .collect();

    let list = List::new(items)
        .block(left_menu)
        .highlight_style(Style::default().bg(Color::Rgb(50, 0, 50)).fg(Color::Black))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.account_menu_index));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let right_content = Block::default()
        .borders(Borders::ALL)
        .title("Content");
    f.render_widget(right_content, chunks[1]);
}
