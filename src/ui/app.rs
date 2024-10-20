use crate::ui::{AppState, draw, Page};
use std::io;
use ratatui::backend::Backend;
use ratatui::Terminal;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::io::stdout;
use ratatui::backend::CrosstermBackend;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use std::time::Duration;

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new(portfolio_data: String) -> Self {
        App {
            state: AppState::new(portfolio_data),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| draw::<B>(f, &self.state))?;

            if event::poll(Duration::from_millis(250))? {
                match event::read()? {
                    CrosstermEvent::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('w') => self.state.set_current_page(Page::WatchList),
                            KeyCode::Char('n') => self.state.set_current_page(Page::TopNftPositions),
                            KeyCode::Char('a') => self.state.set_current_page(Page::Account),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn run_app(app: &mut App) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
