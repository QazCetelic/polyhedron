use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub(crate) fn nvidia_linux(entry: &LogEntry) -> Option<Issue> {
    entry.contents.contains("# C  [libnvidia-glcore.so").then_some(Issue::NvidiaLinux)
}

#[cfg(test)]
mod tests {
    use crate::entries::entry::LogEntry;

    use super::*;

    #[test]
    fn matches_nvidia_linux() {
        let text = r#"[02:57:04] [Render thread/INFO] [KubeJS/]: Loaded client.properties
[02:57:04] [Render thread/INFO] [co.te.re.Redirector/]: Redirecting de/keksuccino/konkrete/config/ConfigEntry$EntryType
[02:57:04] [Render thread/INFO] [co.te.re.Redirector/]: Redirecting net/minecraft/util/ModCheck$Confidence
[02:57:04] [Render thread/INFO] [co.te.re.Redirector/]: Redirecting me/jellysquid/mods/sodium/client/gui/SodiumGameOptions$GraphicsQuality
[02:57:04] [Render thread/INFO] [co.te.re.Redirector/]: Redirecting me/jellysquid/mods/sodium/client/gui/SodiumGameOptions$ArenaMemoryAllocator
#
# A fatal error has been detected by the Java Runtime Environment:
#
#  SIGBUS (0x7) at pc=0x000076b67a7550d5, pid=31, tid=32
#
# JRE version: OpenJDK Runtime Environment Microsoft-11369869 (17.0.15 6) (build 17.0.15 6-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-11369869 (17.0.15 6-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, linux-amd64)
# Problematic frame:
# C  [libnvidia-glcore.so.575.64.03 0xfe77be]
#
# Core dump will be written. Default location: Core dumps may be processed with "/usr/lib/systemd/systemd-coredump %P %u %g %s %t 9223372036854775808 %h %d" (or dumping to /home/********/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/Cisco's Fantasy Medieval RPG [Ultimate]/minecraft/core.31)
#
# An error report file with more information is saved as:
# /home/********/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/Cisco's Fantasy Medieval RPG [Ultimate]/minecraft/hs_err_pid31.log
#
# If you would like to submit a bug report, please visit:
#   https://aka.ms/minecraftjavacrashes
# The crash happened outside the Java Virtual Machine in native code.
# See problematic frame for where to report the bug.
#
REDACTED.
Log upload triggered at: 18 Aug 2025 02:57:31  0300"#;
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| nvidia_linux(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::NvidiaLinux);
    }
}