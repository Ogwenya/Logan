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
