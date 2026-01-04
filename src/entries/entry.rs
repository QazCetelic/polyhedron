use std::str::Lines;

use crate::entries::{parser::LogEntryParser, prefix::LogPrefix};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct LogEntry {
    pub prefix: LogPrefix,
    pub contents: String,
}

impl LogEntry {
    pub fn from_lines(lines: Lines) -> impl Iterator<Item = LogEntry> {
        let mut parser = LogEntryParser::new();
        // Throws in extra line, to indirectly call finalize
        lines.chain(vec![""]).filter_map(move |line| parser.parse_line(line))
    }
}