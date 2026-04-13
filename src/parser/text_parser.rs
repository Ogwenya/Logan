use crate::model::log_entry::LogEntry;
use crate::parser::LogParser;
use std::collections::HashMap;

pub struct TextParser;

impl LogParser for TextParser {
    fn parse(&self, line: &str) -> Option<LogEntry> {
        Some(LogEntry {
            timestamp: None,
            level: None,
            message: line.to_string(),
            service: None,
            fields: HashMap::new(),
        })
    }
}
