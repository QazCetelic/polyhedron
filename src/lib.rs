use std::io::BufRead;

use crate::{entries::{entry::LogEntry, parser::LogEntryParser, prefix::LogPrefix}, header::{identify::LauncherInfo, index::{IndexedLogHeader, LogHeaderIndex}}};


mod entries;
mod header;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadLog {
    pub header: String,
    pub header_index: LogHeaderIndex,
    pub entries: Vec<LogEntry>
}

pub fn read_log<R: BufRead>(mut reader: R) -> Result<ReadLog, ()> {
    let mut lines = reader.lines().peekable();
    let mut header_buffer = String::new();

    while let None = LogPrefix::parse(lines.peek().ok_or(())?.as_ref().map_err(|e| ())?) {
        let line = lines.next().ok_or(())?.map_err(|e| ())?;
        header_buffer.push_str(&line);
        header_buffer.push('\n');
    }
    let index = LogHeaderIndex::from_header(&header_buffer);
    
    let mut entries: Vec<LogEntry> = Vec::new();
    let mut parser =  LogEntryParser::new();
    for line in lines {
        let line = line.map_err(|e| ())?;
        if let Some(entry) = parser.parse_line(&line) {
            entries.push(entry);
        }
    }
    if let Some(entry) = parser.finalize() {
        entries.push(entry);
    }

    Ok(ReadLog {
        header: header_buffer,
        header_index: index,
        entries
    })
}