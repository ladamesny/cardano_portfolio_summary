use std::io;
use std::time::Duration;
use crossterm::event::{self, Event as CEvent, KeyCode};

pub enum Event {
    Input(KeyCode),
    Tick,
    Quit,
}

impl Event {
    pub fn next() -> io::Result<Self> {
        if event::poll(Duration::from_millis(250))? {
            if let CEvent::Key(key) = event::read()? {
                return Ok(match key.code {
                    KeyCode::Char('q') => Event::Quit,
                    KeyCode::Char(c) => Event::Input(KeyCode::Char(c)),
                    other => Event::Input(other),
                });
            }
        }
        Ok(Event::Tick)
    }
}
