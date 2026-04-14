use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};

pub enum AppEvent {
    Quit,
    Down,
    Up,
    Click(u16, u16),
    None,
}

pub fn read_event() -> AppEvent {
    if let Ok(event) = event::read() {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => AppEvent::Quit,
                KeyCode::Down => AppEvent::Down,
                KeyCode::Up => AppEvent::Up,
                _ => AppEvent::None,
            },
            Event::Mouse(mouse) => {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    AppEvent::Click(mouse.column, mouse.row)
                } else {
                    AppEvent::None
                }
            },
            _ => AppEvent::None,
        }
    } else {
        AppEvent::None
    }
}
