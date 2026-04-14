use std::collections::HashMap;
use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Unknown,
}

impl LogLevel {
    pub const ALL: [LogLevel; 6] = [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
        LogLevel::Unknown,
    ];

    pub fn color(&self) -> Color {
        match self {
            LogLevel::Error => Color::Red,
            LogLevel::Warn => Color::Yellow,
            LogLevel::Info => Color::Green,
            LogLevel::Debug => Color::Blue,
            LogLevel::Trace => Color::DarkGray,
            LogLevel::Unknown => Color::Gray,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    #[allow(dead_code)]
    pub timestamp: Option<String>,
    #[allow(dead_code)]
    pub level: Option<LogLevel>,
    pub message: String,
    #[allow(dead_code)]
    pub service: Option<String>,
    #[allow(dead_code)]
    pub fields: HashMap<String, String>,
}

impl LogEntry {
    pub fn get_timestamp(&self) -> String {
        match &self.timestamp {
            Some(t) => t.clone(),
            None => String::from("Unspecified Time"),
        }
    }
}
