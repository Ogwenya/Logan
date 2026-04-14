use crate::model::log_entry::{LogEntry, LogLevel};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub logs: Vec<LogEntry>,
    pub selected: usize,
    pub filter: Option<LogLevel>,
    pub search_query: String,
    pub input_mode: InputMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            selected: 0,
            filter: None,
            search_query: String::new(),
            input_mode: InputMode::Normal,
        }
    }

    pub fn set_filter(&mut self, filter: Option<LogLevel>) {
        self.filter = filter;
        self.selected = 0; // reset selection
    }

    pub fn filtered_logs(&self) -> Vec<&LogEntry> {
        self.logs.iter().filter(|log| {
            let level_match = if let Some(f) = &self.filter {
                log.level.as_ref() == Some(f)
            } else {
                true
            };

            let search_match = if self.search_query.is_empty() {
                true
            } else {
                log.message.to_lowercase().contains(&self.search_query.to_lowercase())
            };

            level_match && search_match
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
