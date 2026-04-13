use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)])
        .split(frame.area());

    let items: Vec<ListItem> = app
        .logs
        .iter()
        .map(|log| ListItem::new(log.message.clone()))
        .collect();

    let list = List::new(items).block(Block::default().title("Logs").borders(Borders::ALL));

    frame.render_widget(list, chunks[0]);
}
