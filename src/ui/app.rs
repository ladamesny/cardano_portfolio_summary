use crate::ui::{AppState, draw, draw_account_page, Page};
use std::io;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
    layout::{Layout, Constraint, Direction},
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        execute,
        event::{self, Event as CrosstermEvent, KeyCode, KeyEvent},
    },
};
use std::io::stdout;
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
            terminal.draw(|f| {
                let size = f.area(); // Get the full terminal size
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                    .split(size);

                if self.state.current_page() == &Page::Account {
                    draw_account_page(f, &self.state, chunks[0]); // Pass the correct Rect
                } else {
                    draw::<B>(f, &self.state);
                }
            })?;

            if event::poll(Duration::from_millis(250))? {
                match event::read()? {
                    CrosstermEvent::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('w') => self.state.set_current_page(Page::WatchList),
                            KeyCode::Char('n') => self.state.set_current_page(Page::TopNftPositions),
                            KeyCode::Char('a') => self.state.set_current_page(Page::Account),
                            KeyCode::Down => {
                                if self.state.current_page() == &Page::Account {
                                    self.state.next_account_menu_item();
                                }
                            },
                            KeyCode::Up => {
                                if self.state.current_page() == &Page::Account {
                                    self.state.previous_account_menu_item();
                                }
                            },
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
