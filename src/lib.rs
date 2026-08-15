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

// We don't need to check the entire log for crashes, because those should only be visible at the end. This improves performance and reduces false positives.
const fn max(a: usize, b: usize) -> usize { [a, b][(a < b) as usize] }
const LAST_ENTRIES_LEN: usize = max(LAST_ENTRIES_STACKTRACES_LEN, max(LAST_ENTRIES_LAST_STACKTRACES_LEN, max(LAST_ENTRIES_LAST_STACKTRACES_LEN, max(LAST_ENTRIES_CRASH_REPORT_LEN, max(LAST_ENTRIES_JRE_LEN, LAST_ENTRIES_CHECKED_LEN)))));
const LAST_ENTRIES_STACKTRACES_LEN: usize = 25;
const LAST_ENTRIES_LAST_STACKTRACES_LEN: usize = 3;
const LAST_ENTRIES_CRASH_REPORT_LEN: usize = 25;
const LAST_ENTRIES_JRE_LEN: usize = 3;
const LAST_ENTRIES_CHECKED_LEN: usize = 10;
const DRAIN_BUFFER: usize = 15; // Prevent draining too often

pub fn read_log<R: BufRead>(reader: R, discard_entries: bool) -> Result<ReadLog, ReadLogError> {
    let mut lines = reader.lines().peekable();
    let _ = lines.peek().ok_or(ReadLogError::Empty)?.as_ref().map_err(|e| ReadLogError::Encoding(e.kind()))?;

    let header = collect_header_string(&mut lines)?;
    let index = LogHeaderIndex::index_header(&header);
    let indexed_header = IndexedLogHeader::from_index(index.clone(), &header);

    let mut issues = Vec::new();
    
    let entries: Vec<LogEntry> = collect_entries(&mut lines, &indexed_header, &mut issues, discard_entries)?;
    let last_entries = entries.get((entries.len().saturating_sub(LAST_ENTRIES_LEN))..).unwrap_or(&entries);

    let crash_report = collect_crash_report(&header, &last_entries);
    let stacktraces = collect_stacktraces(&header, &last_entries);
    let exit_code = last_entries.last().map(|e| extract_exit_code(&e.contents))
        // Check last part of header if no log entries were found
        .unwrap_or_else(|| extract_exit_code(&header.get(header.len().saturating_sub(500)..).unwrap_or(&header)));
    collect_issues(&mut issues, &indexed_header, &entries, crash_report.as_ref(), &stacktraces, exit_code.map(|(_lang, code)| code));

    let jre_fatal_error: Option<JreFatalError> = collect_jre_fatal_error(&header, &entries);
    if let Some(report) = jre_fatal_error.as_ref() {
        issues.push(Issue::FatalErrorJre(Box::new(report.clone())));
    }

    issues.dedup();

    let problematic_mods = collect_problematic_mods(&issues, indexed_header.get_mod_name_lookup_map());
    let recommended_java_version = recommend_java_version(&issues);
    let header_info = LogHeaderInfo::from_indexed_header(&indexed_header);
    let launcher_info = LauncherInfo::from_first_line(&header);

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

fn collect_entries<R: BufRead>(lines: &mut std::iter::Peekable<std::io::Lines<R>>, header: &IndexedLogHeader, issues: &mut Vec<Issue>, discard_entries: bool) -> Result<Vec<LogEntry>, ReadLogError> {
    if discard_entries {
        collect_entries_discard_most(lines, header, issues)
    }
    else {
        collect_entries_discard_none(lines, header, issues)
    }
}

/// Collects entries and scans them for issues while discarding old entries as new come in
fn collect_entries_discard_most<R: BufRead>(lines: &mut std::iter::Peekable<std::io::Lines<R>>, header: &IndexedLogHeader, issues: &mut Vec<Issue>) -> Result<Vec<LogEntry>, ReadLogError> {
    let mut entries: Vec<LogEntry> = Vec::with_capacity(LAST_ENTRIES_LEN + DRAIN_BUFFER);
    let mut parser =  LogEntryParser::new();

    fn add_entry(entries: &mut Vec<LogEntry>, entry: Option<LogEntry>, header: &IndexedLogHeader, issues: &mut Vec<Issue>) {
        if let Some(entry) = entry {
            entries.push(entry);
            if entries.len() > (LAST_ENTRIES_LEN + DRAIN_BUFFER) {
                let to_keep = entries.len().saturating_sub(LAST_ENTRIES_LEN);
                let entries = entries.drain(..to_keep).collect::<Vec<LogEntry>>();
                collect_issues_all_entries(issues, header, &entries);
            }
        }
    }

    for line in lines {
        let line = line.map_err(|e| ReadLogError::Encoding(e.kind()))?;
        add_entry(&mut entries, parser.parse_line(&line), header, issues);
    }
    add_entry(&mut entries, parser.finalize(), header, issues);
    collect_issues_all_entries(issues, header, &entries);

    Ok(entries)
}

// Collects all entries and then scans them for issues in a single pass
fn collect_entries_discard_none<R: BufRead>(lines: &mut std::iter::Peekable<std::io::Lines<R>>, header: &IndexedLogHeader, issues: &mut Vec<Issue>) -> Result<Vec<LogEntry>, ReadLogError> {
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
    collect_issues_all_entries(issues, header, &entries);
    Ok(entries)
}

fn collect_jre_fatal_error(header: &str, entries: &[LogEntry]) -> Option<JreFatalError> {
    if let Some(report) = JreFatalError::parse(&header) {
        return Some(report);
    }
    for entry in entries.iter().rev().take(LAST_ENTRIES_JRE_LEN) {
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
    for entry in entries.iter().rev().take(LAST_ENTRIES_STACKTRACES_LEN) {
        let stacktrace_iter = Stacktrace::from_lines(entry.contents.lines());
        for stacktrace in stacktrace_iter {
            stacktraces.push(stacktrace);
        }
    }
    return stacktraces;
}

fn collect_header_string<R: BufRead>(lines: &mut std::iter::Peekable<std::io::Lines<R>>) -> Result<String, ReadLogError> {
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
    for entry in entries.iter().rev().take(LAST_ENTRIES_CRASH_REPORT_LEN) {
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

fn collect_issues(issues: &mut Vec<Issue>, header: &IndexedLogHeader<'_>, entries: &[LogEntry], crash_report: Option<&CrashReport>, stacktraces: &[Stacktrace], exit_code: Option<i32>) {
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
        let last_stacktraces: Vec<Stacktrace> = stacktraces.iter().rev().take(LAST_ENTRIES_LAST_STACKTRACES_LEN).map(|st| st.clone()).collect();
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

    for build_entry_check in CHECKS_LAST_ENTRIES {
        let entry_check = build_entry_check(header);
        for entry in entries.iter().rev().take(LAST_ENTRIES_CHECKED_LEN) {
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
}