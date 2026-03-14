use std::{collections::BTreeSet, io::{BufRead, ErrorKind}};

use thiserror::Error;

use crate::{entries::{entry::LogEntry, parser::LogEntryParser, prefix::LogPrefix}, header::{identify::LauncherInfo, index::{IndexedLogHeader, LogHeaderIndex}, info::LogHeaderInfo}, issues::{conclude::{RecommendedJavaVersion, collect_problematic_mods, recommend_java_version}, groups::{CHECKS_ALL_STACKTRACES, CHECKS_CRASH_REPORT, CHECKS_ENTRIES, CHECKS_EXIT_CODE, CHECKS_HEADER, CHECKS_LAST_ENTRIES, CHECKS_LAST_STACKTRACES, CHECKS_TEXT}, issue::Issue}, parse::{crash_report::CrashReport, exit_code::extract_exit_code, jre_fatal::JreFatalError, stacktrace::model::Stacktrace}};

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
    pub localization: Option<String>,
    pub exit_code: Option<i32>,
    pub problematic_mods: BTreeSet<String>,
    pub recommended_java_version: Option<RecommendedJavaVersion>,
}

pub fn read_log<R: BufRead>(reader: R) -> Result<ReadLog, ReadLogError> {
    let mut lines = reader.lines().peekable();
    let first_line = lines.peek().ok_or(ReadLogError::Empty)?.as_ref().map_err(|e| ReadLogError::Encoding(e.kind()))?;
    let launcher_info = LauncherInfo::from_first_line(&first_line);

    let header = collect_header(&mut lines)?;

    let index = LogHeaderIndex::index_header(&header);
    let indexed_header = IndexedLogHeader::from_index(index.clone(), &header);
    
    let entries: Vec<LogEntry> = collect_all_entries(&mut lines)?;
    let crash_report = collect_crash_report(&header, &entries);
    let stacktraces = collect_stacktraces(&header, &entries);

    let exit_code = entries.last().map(|e| extract_exit_code(&e.contents)).unwrap_or_else(|| extract_exit_code(&header));
    let mut issues = collect_issues(&indexed_header, &entries, crash_report.as_ref(), &stacktraces, exit_code.map(|(_lang, code)| code));

    let jre_fatal_error: Option<JreFatalError> = collect_jre_fatal_error(&header, &entries);
    if let Some(report) = jre_fatal_error.as_ref() {
        issues.push(Issue::FatalErrorJre(Box::new(report.clone())));
    }

    issues.dedup();

    let problematic_mods = collect_problematic_mods(&issues, indexed_header.get_mod_name_lookup_map());
    let recommended_java_version = recommend_java_version(&issues);
    let header_info = LogHeaderInfo::from_indexed_header(&indexed_header);

    Ok(ReadLog {
        launcher_info,
        header,
        header_info: header_info,
        header_index: index,
        entries,
        issues,
        stacktraces,
        crash_report,
        jre_fatal_error,
        localization: exit_code.map(|(lang, _code)| lang.to_string()),
        exit_code: exit_code.map(|(_lang, code)| code),
        problematic_mods,
        recommended_java_version,
    })
}

fn collect_all_entries<R: BufRead>(lines: &mut std::iter::Peekable<std::io::Lines<R>>) -> Result<Vec<LogEntry>, ReadLogError> {
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
    Ok(entries)
}

fn collect_jre_fatal_error(header: &str, entries: &[LogEntry]) -> Option<JreFatalError> {
    if let Some(report) = JreFatalError::parse(&header) {
        return Some(report);
    }
    for entry in entries.iter().rev().take(3) { // We don't check that far because this should always be at the bottom
        if let Some(report) = JreFatalError::parse(&entry.contents) {
            return Some(report);
        }
    }
    None
}

fn collect_stacktraces(header: &str, entries: &[LogEntry]) -> Vec<Stacktrace> {
    let mut stacktraces: Vec<Stacktrace> = Vec::new();
    let stacktrace_iter = Stacktrace::from_lines(header.lines());
    for stacktrace in stacktrace_iter {
        stacktraces.push(stacktrace);
    }
    for entry in entries.iter().rev().take(25) {
        let stacktrace_iter = Stacktrace::from_lines(entry.contents.lines());
        for stacktrace in stacktrace_iter {
            stacktraces.push(stacktrace);
        }
    }
    return stacktraces;
}

fn collect_header<R: BufRead>(lines: &mut std::iter::Peekable<std::io::Lines<R>>) -> Result<String, ReadLogError> {
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
    Ok(header_buffer)
}

fn collect_crash_report(header: &str, entries: &[LogEntry]) -> Option<CrashReport> {
    if let Some(report) = CrashReport::parse(header) {
        return Some(report);
    }
    for entry in entries.iter().rev().take(75) {
        if let Some(report) = CrashReport::parse(&entry.contents) {
            return Some(report);
        }
    }
    None
}

fn collect_issues_all_entries(issues: &mut Vec<Issue>, header: &IndexedLogHeader, entries: &[LogEntry]) {
    for build_entry_check in CHECKS_ENTRIES {
        let entry_check = build_entry_check(header);
        for entry in entries {
            if let Some(issue) = entry_check(entry) {
                issues.push(issue);
            }
        }
    }
}

fn collect_issues(header: &IndexedLogHeader<'_>, entries: &[LogEntry], crash_report: Option<&CrashReport>, stacktraces: &[Stacktrace], exit_code: Option<i32>) -> Vec<Issue> {
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
        for build_last_stacktrace_check in CHECKS_LAST_STACKTRACES {
            let crash_report_check = build_last_stacktrace_check(header);
            if let Some(issue) = crash_report_check(&report.stacktrace) {
                issues.push(issue);
            }
        }
    }
    else {
        let last_stacktraces: Vec<Stacktrace> = stacktraces.iter().rev().take(3).map(|st| st.clone()).collect();
        for build_last_stacktrace_check in CHECKS_LAST_STACKTRACES {
            let crash_report_check: Box<dyn Fn(&[Stacktrace]) -> Option<Issue>> = build_last_stacktrace_check(header);
            if let Some(issue) = crash_report_check(&last_stacktraces) {
                issues.push(issue);
            }
        }
    }

    for stacktrace in stacktraces {
        for stacktrace_check in CHECKS_ALL_STACKTRACES  {
            if let Some(issue) = stacktrace_check(&stacktrace) {
                issues.push(issue);
            }
        }
    }

    collect_issues_all_entries(&mut issues, header, entries);
    for build_entry_check in CHECKS_LAST_ENTRIES {
        let entry_check = build_entry_check(header);
        for entry in entries.iter().rev().take(10) {
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

    if let Some(code) = exit_code {
        for build_exit_code_check in CHECKS_EXIT_CODE {
            let exit_code_check = build_exit_code_check(header);
            if let Some(issue) = exit_code_check(code) {
                issues.push(issue);
            }
        }
    }

    issues
}