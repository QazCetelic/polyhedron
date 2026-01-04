use crate::{entries::entry::LogEntry, issues::issue::Issue};

fn checksum_mismatch(text: &str) -> Option<Issue> {
    text.contains("Checksum mismatch, download is bad.").then_some(Issue::ChecksumMismatch)
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
        let issue = checksum_mismatch(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::ChecksumMismatch);
    }
}