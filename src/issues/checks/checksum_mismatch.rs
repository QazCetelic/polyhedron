use crate::{entries::entry::LogEntry, issues::issue::Issue};

// pub fn checksum_mismatch_text(text: &str) -> Option<Issue> {
//     text.contains("Checksum mismatch, download is bad.").then_some(Issue::ChecksumMismatch)
// }

pub(crate) fn checksum_mismatch_entry(entry: &LogEntry) -> Option<Issue> {
    entry.contents.starts_with("Checksum mismatch, download is bad.").then_some(Issue::ChecksumMismatch)
}

#[cfg(test)]
mod tests {
    use crate::entries::entry::LogEntry;

    use super::*;

    #[test]
    fn matches_checksum_mismatch() {
        let text = r#" 35911.334 W | beginResetModel called on VersionProxyModel(0x16b80b61970) without calling endResetModel first
 35911.334 W | endResetModel called on VersionProxyModel(0x16b80b61970) without calling beginResetModel first
 35911.847 W | Checksum mismatch, download is bad.
 35911.964 W | Checksum mismatch, download is bad.
 35912.081 W | Checksum mismatch, download is bad.
 35912.100 W | "One or more subtasks failed""#;
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| checksum_mismatch_entry(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::ChecksumMismatch);
    }
}