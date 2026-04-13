use crate::model::log_entry::{LogEntry, LogLevel};
use crate::parser::LogParser;
use std::collections::HashMap;

pub struct JsonParser;

impl LogParser for JsonParser {
    fn parse(&self, line: &str) -> Option<LogEntry> {
        let value: serde_json::Value = serde_json::from_str(line).ok?;

        let obj = value.as_object()?;

        some(LogEntry {
            timestamp: obj.get("timestamp")?.as_str().map(String::from),
            level: obj.get("level").and_then(|v| v.as_str()).map(parse_level),
            message: obj.get("message")?.as_str()?.to_string(),
            service: obj
                .get("service")
                .and_then(|v| v.as_str())
                .map(String::from),
            fields: obj
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect::<HashMap<_, _>>(),
        })
    }
}

fn parse_level(level: &str) {
    match level.to_lowercase().as_str() {
        "trace" => LogLevel::Trace,
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => LogLevel::Unknown,
    }
}
