pub mod json_parser;
pub mod text_parser;

use crate::model::log_entry::LogEntry;

pub trait LogParser {
    fn parse(&self, line: &str) -> Option<LogEntry>;
}
