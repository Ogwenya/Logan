use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;
use crate::model::log_entry::LogLevel;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(frame.area());

    let mut sidebar_items = vec![];
    let reset_text = if app.filter.is_none() { ">> Reset <<" } else { "Reset" };
    sidebar_items.push(ListItem::new(reset_text));

    for level in LogLevel::ALL {
        let text = format!("{:?}", level);
        let item = if app.filter.as_ref() == Some(&level) {
            ListItem::new(format!(">> {} <<", text))
        } else {
            ListItem::new(text)
        };
        sidebar_items.push(item);
    }

    let sidebar = List::new(sidebar_items)
        .block(Block::default().title("Levels").borders(Borders::ALL));
    
    frame.render_widget(sidebar, chunks[0]);

    let filtered_logs = app.filtered_logs();
    let log_items: Vec<ListItem> = filtered_logs
        .iter()
        .map(|log| ListItem::new(log.message.clone()))
        .collect();

    let logs_list = List::new(log_items).block(Block::default().title("Logs").borders(Borders::ALL));

    frame.render_widget(logs_list, chunks[1]);
}
