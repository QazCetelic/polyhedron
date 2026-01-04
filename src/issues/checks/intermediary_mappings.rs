use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub(crate) fn intermediary_mappings(entry: &LogEntry) -> Option<Issue> {
    entry.contents.starts_with("Mapping source name conflicts detected:").then_some(Issue::WrongIntermediaryMappingsVersion)
}

#[cfg(test)]
mod tests {
    use crate::entries::entry::LogEntry;

    use super::*;

    #[test]
    fn matches_intermediary_mappings() {
        let text = r#"[20:36:34] [INFO] [FabricLoader/GameProvider]: Loading Minecraft 1.21.10 with Fabric Loader 0.18.1
[20:36:34] [WARN] [FabricLoader/GameRemap]: Incomplete remapped file found! This means that the remapping process failed on the previous launch. If this persists, make sure to let us at Fabric know!
[20:36:34] [INFO] [FabricLoader/GameRemap]: Fabric is preparing JARs on first launch, this may take a few seconds...
[20:36:36] [WARN] [FabricLoader/GameRemap]: Mapping source name conflicts detected:
[20:36:36] [WARN] [FabricLoader/GameRemap]: ehp METHOD b (()I) -> [ehp/method_12031, ego/method_12197]
"#;
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| intermediary_mappings(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::WrongIntermediaryMappingsVersion);
    }
}