use std::io::{BufRead, ErrorKind};

use thiserror::Error;

use crate::{entries::{entry::LogEntry, parser::LogEntryParser, prefix::LogPrefix}, header::{identify::LauncherInfo, index::{IndexedLogHeader, LogHeaderIndex}, info::LogHeaderInfo}};

mod entries;
mod header;

#[derive(Error, Debug)]
pub enum ReadLogError {
    #[error("Failed to decode text")]
    Encoding(ErrorKind),
    #[error("Log is empty")]
    Empty,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadLog {
    pub launcher_info: Option<LauncherInfo>,
    pub header: String,
    pub header_info: LogHeaderInfo,
    pub header_index: LogHeaderIndex,
    pub entries: Vec<LogEntry>
}

pub fn read_log<R: BufRead>(mut reader: R) -> Result<ReadLog, ReadLogError> {
    let mut lines = reader.lines().peekable();
    let first_line = lines.peek().ok_or(ReadLogError::Empty)?.as_ref().map_err(|e| ReadLogError::Encoding(e.kind()))?;
    let launcher_info = LauncherInfo::from_first_line(&first_line);

    let mut header_buffer = String::new();

    loop {
        if let Some(lr) = lines.peek() {
            let line = lr.as_ref().map_err(|e| ReadLogError::Encoding(e.kind()))?;
            if let None = LogPrefix::parse(&line) {
                header_buffer.push_str(&line);
                header_buffer.push('\n');
                lines.next();
                continue;
            }
        }
        break;
    }
    let index = LogHeaderIndex::index_header(&header_buffer);
    let indexed_header = IndexedLogHeader::from_index(index.clone(), &header_buffer);
    let header_info = LogHeaderInfo::from_indexed_header(&indexed_header);
    
    let mut entries: Vec<LogEntry> = Vec::new();
    let mut parser =  LogEntryParser::new();
    for line in lines {
        let line = line.map_err(|e| ReadLogError::Encoding(e.kind()))?;
        if let Some(entry) = parser.parse_line(&line) {
            entries.push(entry);
        }
    }
    if let Some(entry) = parser.finalize() {
        entries.push(entry);
    }

    Ok(ReadLog {
        launcher_info,
        header: header_buffer,
        header_info: header_info,
        header_index: index,
        entries
    })
}