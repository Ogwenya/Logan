use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};

use std::io;

use app::App;
use parser::{LogParser, json_parser::JsonParser, text_parser::TextParser};
use source::file_source::FileSource;
use tui::{events, ui};

use crate::source::LogSource;

mod app;
mod model;
mod parser;
mod source;
mod tui;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let _ = execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        Clear(ClearType::All)
    );

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let mut source = FileSource::new("example.log");

    let json_parser = JsonParser;
    let text_parser = TextParser;

    while let Some(line) = source.read_line() {
        let entry = json_parser
            .parse(&line)
            .or_else(|| text_parser.parse(&line));

        if let Some(log) = entry {
            app.logs.push(log);
        }
    }

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        match events::read_event() {
            events::AppEvent::Quit => break,
            events::AppEvent::Down => app.next(),
            events::AppEvent::Up => app.previous(),
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
