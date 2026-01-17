use std::io::{BufRead, ErrorKind};

use thiserror::Error;

use crate::{entries::{entry::LogEntry, parser::LogEntryParser, prefix::LogPrefix}, header::{identify::LauncherInfo, index::{IndexedLogHeader, LogHeaderIndex}, info::LogHeaderInfo}, issues::{checks::{CHECKS_CRASH_REPORT, CHECKS_ENTRIES, CHECKS_HEADER, CHECKS_TEXT}, issue::Issue}, parse::crash_report::CrashReport};

mod entries;
mod header;
mod issues;
mod parse;

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
    pub entries: Vec<LogEntry>,
    pub issues: Vec<Issue>,
    pub crash_report: Option<CrashReport>
}

pub fn read_log<R: BufRead>(reader: R) -> Result<ReadLog, ReadLogError> {
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

    let header_crash_report = CrashReport::parse(&header_buffer);
    
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

    let indexed_header = IndexedLogHeader::from_index(index.clone(), &header_buffer);
    let mut issues = find_issues(&indexed_header, &entries);

    let mut crash_report = header_crash_report;
    for entry in entries.iter().rev().take(75) {
        if let Some(report) = CrashReport::parse(&entry.contents) {
            crash_report = Some(report);
            break;
        }
    }

    if let Some(report) = &crash_report {
        for crash_report_check in CHECKS_CRASH_REPORT {
            if let Some(issue) = crash_report_check(&report) {
                issues.push(issue);
            }
        }
    }

    Ok(ReadLog {
        launcher_info,
        header: header_buffer,
        header_info: header_info,
        header_index: index,
        entries,
        issues,
        crash_report,
    })
}

fn find_issues(header: &IndexedLogHeader<'_>, entries: &[LogEntry]) -> Vec<Issue> {
    let mut issues = Vec::new();
    
    for header_check in CHECKS_HEADER {
        if let Some(issue) = header_check(header) {
            issues.push(issue);
        }
    }

    for build_entry_check in CHECKS_ENTRIES {
        let entry_check = build_entry_check(header);
        for entry in entries {
            if let Some(issue) = entry_check(entry) {
                issues.push(issue);
            }
        }
    }

    let checks_text = CHECKS_TEXT.map(|c| c(header));
    for text_check in checks_text {
        if let Some(issue) = text_check(header.text) {
            issues.push(issue);
        }
        for entry in entries {
            if let Some(issue) = text_check(&entry.contents) {
                issues.push(issue);
            }
        }
    }

    issues
}