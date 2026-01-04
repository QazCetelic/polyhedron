use std::str::Lines;

use crate::entries::{parser::LogEntryParser, prefix::LogPrefix};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct LogEntry {
    pub prefix: LogPrefix,
    pub contents: String,
}

impl LogEntry {
    pub fn from_lines(lines: Lines) -> Vec<LogEntry> {
        let mut parser = LogEntryParser::new();
        let mut entries = Vec::new();
        for line in lines {
            if let Some(entry) = parser.parse_line(line) {
                entries.push(entry);
            }
        }
        if let Some(entry) = parser.finalize() {
            entries.push(entry);
        }
        entries
    }
}