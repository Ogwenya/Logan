use crossterm::event::{self, Event, KeyCode};

pub enum AppEvent {
    Quit,
    Down,
    Up,
    None,
}

pub fn read_event() -> AppEvent {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Char('q') => AppEvent::Quit,
            KeyCode::Down => AppEvent::Down,
            KeyCode::Up => AppEvent::Up,
            _ => AppEvent::None,
        }
    } else {
        AppEvent::None
    }
}
