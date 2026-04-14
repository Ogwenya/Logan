use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, InputMode};
use crate::model::log_entry::LogLevel;

pub fn draw(frame: &mut Frame, app: &App) {
    let parent_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let search_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    };

    let search_title = match app.input_mode {
        InputMode::Normal => "Search (Press '/' to search, 'q' to quit)",
        InputMode::Editing => "Search (Press 'Esc' or 'Enter' to stop editing)",
    };

    let search_block = Paragraph::new(app.search_query.clone()).block(
        Block::default()
            .title(search_title)
            .borders(Borders::ALL)
            .style(search_style),
    );
    frame.render_widget(search_block, parent_chunks[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(parent_chunks[1]);

    let mut sidebar_items = vec![];
    let reset_text = if app.filter.is_none() {
        ">> Reset <<"
    } else {
        "Reset"
    };
    sidebar_items.push(ListItem::new(reset_text));

    for level in LogLevel::ALL {
        let text = format!("{:?}", level);
        let color = level.color();
        let item = if app.filter.as_ref() == Some(&level) {
            ListItem::new(format!(">> {} <<", text)).style(Style::default().fg(color))
        } else {
            ListItem::new(text).style(Style::default().fg(color))
        };
        sidebar_items.push(item);
    }

    let sidebar =
        List::new(sidebar_items).block(Block::default().title("Levels").borders(Borders::ALL));

    frame.render_widget(sidebar, chunks[0]);

    let filtered_logs = app.filtered_logs();
    let log_items: Vec<ListItem> = filtered_logs
        .iter()
        .map(|log| {
            let timestamp_span = Span::styled(
                format!("{} : ", log.get_timestamp()),
                Style::default().fg(Color::DarkGray),
            );
            
            let message_span = Span::styled(
                log.message.clone(),
                Style::default().fg(Color::White),
            );

            ListItem::new(Line::from(vec![timestamp_span, message_span]))
        })
        .collect();

    let logs_list =
        List::new(log_items).block(Block::default().title("Logs").borders(Borders::ALL));

    frame.render_widget(logs_list, chunks[1]);
}
