use crate::model::log_entry::{LogEntry, LogLevel};

pub struct App {
    pub logs: Vec<LogEntry>,
    pub selected: usize,
    pub filter: Option<LogLevel>,
}

impl App {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            selected: 0,
            filter: None,
        }
    }

    pub fn set_filter(&mut self, filter: Option<LogLevel>) {
        self.filter = filter;
        self.selected = 0; // reset selection
    }

    pub fn filtered_logs(&self) -> Vec<&LogEntry> {
        self.logs.iter().filter(|log| {
            if let Some(f) = &self.filter {
                log.level.as_ref() == Some(f)
            } else {
                true
            }
        }).collect()
    }

    pub fn next(&mut self) {
        if self.selected < self.filtered_logs().len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}
