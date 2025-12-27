use crate::entries::{entry::LogEntry, prefix::LogPrefix};


#[allow(dead_code)]
pub struct LogEntryParser {
    prefix: Option<LogPrefix>,
    contents: String,
}

#[allow(dead_code)]
impl LogEntryParser {
    pub fn new() -> Self {
        Self {
            prefix: None,
            contents: String::new(),
        }
    }

    /// Parses a single line of the log and returns a completed LogEntry if a new entry starts.
    pub fn parse_line(&mut self, line: &str) -> Option<LogEntry> {
        if let Some((new_prefix, rest_of_line)) = LogPrefix::parse(line) { // New prefix? Create new entry and return previous
            let completed_entry = if let Some(existing_prefix) = &self.prefix {
                Some(LogEntry {
                    prefix: existing_prefix.clone(),
                    contents: self.contents.clone(),
                })
            } else {
                None
            };
            self.prefix = Some(new_prefix);
            self.contents = rest_of_line.to_string();
            completed_entry
        } else { // Add to existing entry
            if !self.contents.is_empty() {
                self.contents.push('\n');
            }
            self.contents.push_str(line);
            None
        }
    }

    /// Finalizes the parser and returns the last log entry if present.
    pub fn finalize(&mut self) -> Option<LogEntry> {
        if let Some(existing_prefix) = &self.prefix {
            Some(LogEntry {
                prefix: existing_prefix.clone(),
                contents: self.contents.clone(),
            })
        } else {
            None
        }
    }
}