use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub(crate) fn old_java_macos(entry: &LogEntry) -> Option<Issue> {
   entry.contents.contains("~StubRoutines::SafeFetch32").then_some(Issue::OldJavaMacOs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_old_java_macos() {
        let text = r#"[15:11:41] [Server thread/INFO] [Puzzles Lib/]: Loading server config for easyanvils
[15:11:41] [Server thread/INFO] [co.to.st.StorageMod/]: Loaded Tom's Simple Storage config file toms_storage-server.toml
[15:11:41] [Server thread/INFO] [FluxNetworks/Energy]: Energy blacklist loaded: 1 block entries, 0 item entries
[15:11:41] [Server thread/INFO] [spark/]: Starting background profiler...
#
# A fatal error has been detected by the Java Runtime Environment:
#
#  SIGBUS (0xa) at pc=0x000000010d4c54e4, pid=57376, tid=237059
#
# JRE version: OpenJDK Runtime Environment Microsoft-8035246 (17.0.8 7) (build 17.0.8 7-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-8035246 (17.0.8 7-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, bsd-aarch64)
# Problematic frame:
# v  ~StubRoutines::SafeFetch32
#
# No core dump will be written. Core dumps have been disabled. To enable core dumping, try "ulimit -c unlimited" before starting Java again
#"#;
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| old_java_macos(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::OldJavaMacOs);
    }
}