use crate::model::log_entry::LogEntry;

pub struct App {
    pub logs: Vec<LogEntry>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            selected: 0,
        }
    }

    pub fn next(&mut self) {
        if self.selected < self.logs.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}
