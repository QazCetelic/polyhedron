use std::collections::BTreeMap;

use crate::entries::time::LogTime;

#[derive(Debug)]
struct CrashReport {
    time: LogTime,
    description: String,
    stacktrace: String, // TODO parse stacktrace
    sections: BTreeMap<String, String>
}

impl CrashReport {
    fn parse(text: &str) -> Option<CrashReport> {
        let (_, report_etc) = text.split_once("---- Minecraft Crash Report ----")?;
        let (remarks, time_etc) = report_etc.split_once("Time: ")?;
        // The remarks section sometimes contains additional comments about e.g. coremods that may be useful
        let (time_str, description_etc) = time_etc.split_once("Description: ")?;
        let time_str = time_str.trim_ascii_end();
        let time = LogTime::parse(time_str)?; // Time can be in various formats
        let (description, stacktrace_etc) = description_etc.split_once('\n')?;
        let description = description.trim_ascii_end().to_string();
        let (stacktrace, etc) = stacktrace_etc.split_once("A detailed walkthrough of the error, its code path and all known details is as follows:")?;
        let stacktrace = stacktrace.trim_ascii().to_string();
        let (_, etc) = etc.trim_ascii_start().split_once("---------------------------------------------------------------------------------------")?;
        let sections = split_sections(etc);

        let report = CrashReport {
            time,
            description,
            stacktrace,
            sections,
        };

        Some(report)
    }
}



fn split_sections(s: &str) -> BTreeMap<String, String> { // The sections appear to be dynamic so I don't think I can use the same strategy as IndexedHeader
    let mut lines = s.lines();
    let mut sections: BTreeMap<String, String> = Default::default();
    let mut text = String::new();
    let mut header: Option<String> = None;
    while let Some(line) = lines.next() {
        if line.starts_with("#@!@# Game crashed! Crash report saved to: #@!@#") { // This may be missing
            break;
        }
        if let Some(header_text) = line.strip_prefix("-- ").map(|l| l.strip_suffix(" --")).flatten() 
           && header_text.chars().all(|c| c == ' ' || c.is_ascii_alphabetic()) {
            if let Some(s) = header {
                sections.insert(s, text);
                text = String::new();
            }
            header = Some(header_text.to_string());
        }
        else if let Some(_) = &header {
            text.push_str(line);
            text.push('\n');
        }
    }
    if let Some(s) = header {
        sections.insert(s, text);
    }
    sections
}

#[cfg(test)]
mod tests {
    use crate::parse::{crash_report::CrashReport, section_tree::SectionTree};


    #[test]
    fn simple() {
        let text = include_str!("test_data/crash_1.log");
        let report = CrashReport::parse(text).expect("Failed to parse crash report");
        let sytem_details = report.sections.get("System Details").expect("Failed to get System Details section");
        let tree = SectionTree::parse(sytem_details).expect("Failed to parse system details");
    }
}
