use std::collections::HashMap;

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
    fn get_timestamp(&self) -> String {
        match &self.timestamp {
            Some(t) => t.clone(),
            None => String::from("Unspecified Time"),
        }
    }

    pub fn generate(&self) -> String {
        format!("{} : {}", self.get_timestamp(), &self.message)
    }
}
