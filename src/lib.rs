use std::io::{BufRead, ErrorKind};

use thiserror::Error;

use crate::{entries::{entry::LogEntry, parser::LogEntryParser, prefix::LogPrefix}, header::{identify::LauncherInfo, index::{IndexedLogHeader, LogHeaderIndex}, info::LogHeaderInfo}, issues::{checks::{CHECKS_CRASH_REPORT, CHECKS_ENTRIES, CHECKS_HEADER, CHECKS_STACKTRACE, CHECKS_TEXT}, issue::Issue}, parse::{crash_report::CrashReport, jre_fatal::JreFatalError, stacktrace::Stacktrace}};

pub mod entries;
pub mod header;
pub mod issues;
pub mod parse;

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
    pub stacktraces: Vec<Stacktrace>,
    pub crash_report: Option<CrashReport>,
    pub jre_fatal_error: Option<JreFatalError>,
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

    let mut crash_report = header_crash_report;
    for entry in entries.iter().rev().take(75) {
        if let Some(report) = CrashReport::parse(&entry.contents) {
            crash_report = Some(report);
            break;
        }
    }

    let mut stacktraces = Vec::new();
    for entry in entries.iter().rev().take(25) {
        let stacktrace_iter = Stacktrace::from_lines(entry.contents.lines());
        for stacktrace in stacktrace_iter {
            stacktraces.push(stacktrace);
        }
    }

    let indexed_header = IndexedLogHeader::from_index(index.clone(), &header_buffer);
    let mut issues = find_issues(&indexed_header, &entries, crash_report.as_ref(), &stacktraces);

    let mut jre_fatal_error: Option<JreFatalError> = None;
    for entry in entries.iter().rev().take(3) { // We don't check that far because this should always be at the bottom
        if let Some(report) = JreFatalError::parse(&entry.contents) {
            jre_fatal_error = Some(report.clone());
            issues.push(Issue::FatalErrorJre(Box::new(report)));
            break;
        }
    }

    Ok(ReadLog {
        launcher_info,
        header: header_buffer,
        header_info: header_info,
        header_index: index,
        entries,
        issues,
        stacktraces,
        crash_report,
        jre_fatal_error,
    })
}

pub fn find_exception_locations<R: BufRead>(mut reader: R) -> Option<Vec<String>> {
    let mut text = String::new();
    reader.read_to_string(&mut text).ok()?;
    let stacktraces = Stacktrace::from_lines(text.lines());
    let lines = stacktraces
        .map(|s| s.lines)
        .flatten()
        .filter_map(|l| l.get_relative_path())
        .map(|(path, line)| format!("{path}:{line}"))
        .collect::<Vec<String>>();

    Some(lines)
}

fn find_issues(header: &IndexedLogHeader<'_>, entries: &[LogEntry], crash_report: Option<&CrashReport>, stacktraces: &[Stacktrace]) -> Vec<Issue> {
    let mut issues = Vec::new();
    
    for header_check in CHECKS_HEADER {
        if let Some(issue) = header_check(header) {
            issues.push(issue);
        }
    }

    if let Some(report) = crash_report {
        for build_crash_report_check in CHECKS_CRASH_REPORT {
            let crash_report_check = build_crash_report_check(header);
            if let Some(issue) = crash_report_check(&report) {
                issues.push(issue);
            }
        }
    }

    for stacktrace in stacktraces {
        for stacktrace_check in CHECKS_STACKTRACE  {
            if let Some(issue) = stacktrace_check(&stacktrace) {
                issues.push(issue);
            }
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