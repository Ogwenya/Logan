use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};

use std::{env, io, path::Path};

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

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), io::Error> {
    let mut source = open_file(env::args())?;

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
            events::AppEvent::Char(c) => {
                match app.input_mode {
                    app::InputMode::Normal => {
                        if c == 'q' {
                            break;
                        } else if c == '/' {
                            app.input_mode = app::InputMode::Editing;
                        }
                    }
                    app::InputMode::Editing => {
                        app.search_query.push(c);
                        app.selected = 0;
                    }
                }
            }
            events::AppEvent::Backspace => {
                if let app::InputMode::Editing = app.input_mode {
                    app.search_query.pop();
                    app.selected = 0;
                }
            }
            events::AppEvent::Enter | events::AppEvent::Esc => {
                app.input_mode = app::InputMode::Normal;
            }
            events::AppEvent::Down => {
                if let app::InputMode::Normal = app.input_mode {
                    app.next();
                }
            }
            events::AppEvent::Up => {
                if let app::InputMode::Normal = app.input_mode {
                    app.previous();
                }
            }
            events::AppEvent::Click(col, row) => {
                // Adjust row click for layout shift (top search bar takes 3 units)
                if row >= 3 && col < 20 {
                    let adjusted_row = row - 3;
                    if adjusted_row == 1 {
                        app.set_filter(None);
                    } else if adjusted_row >= 2 {
                        let idx = adjusted_row as usize - 2;
                        if idx < crate::model::log_entry::LogLevel::ALL.len() {
                            app.set_filter(Some(crate::model::log_entry::LogLevel::ALL[idx].clone()));
                        }
                    }
                }
            }
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

fn open_file(mut args: impl Iterator<Item = String>) -> Result<FileSource, io::Error> {
    args.next();

    match args.next() {
        Some(arg) => {
            let is_file = Path::new(&arg).is_file();

            if is_file {
                Ok(FileSource::new(&arg))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "The provided path is not a valid file path",
                ))
            }
        }
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Provide a file path",
        )),
    }
}
