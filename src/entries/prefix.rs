use crate::entries::time::LogPrefixTime;


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LogPrefix {
    /// Timestamp of the log entry, optionally including date and milliseconds
    pub time: LogPrefixTime,
    /// Thread / source from which the log originated
    pub thread: String,
    /// Log level like INFO, WARN, ERROR
    pub level: String,
    /// Optional context from where the log originated, like a mixin or class name
    pub context: Option<String>,
}

// Not fully to spec, but functional
fn basic_strip_ansi_escape(mut line: &str) -> &str {
    while let Some(escape_pos) = line.find(27 as char) {
        let end_pos = line.get(escape_pos..).map(|l| l.find('m')).flatten();
        match end_pos {
            Some(pos) => {
                if let Some(l) = line.get(pos + 1..) {
                    line = l;
                }
            },
            None => {
                if let Some(l) = line.get(1..) {
                    line = l;
                }
            },
        }
    }
    line
}

// "2025-10-30T19:21:06.036061Z main WARN Advanced terminal features are not available in this environment"
fn parse_iso_no_brackets(line: &str) -> Option<(LogPrefix, &str)> {
    let (time_str, rest) = line.split_once(' ')?;
    let time = LogPrefixTime::parse(time_str)?; // "2025-10-30T19:21:06.036061Z"
    let (thread_str, rest) = rest.split_once(' ')?;
    let (level_str, rest) = rest.split_once(' ')?;
    let prefix = LogPrefix {
        time,
        thread: thread_str.to_string(),
        level: level_str.to_string(),
        context: None,
    };
    Some((prefix, rest))
}

// "[16:20:50] [Client thread/INFO]: LWJGL Version: 2.9.4"
// "[17:26:36.877] [main/INFO] [loading.moddiscovery.ModDiscoverer/SCAN]: Found mod file..."
fn parse_with_brackets(line: &str) -> Option<(LogPrefix, &str)> {
    let line = line.strip_prefix('[')?;
    let (time_str, rest) = line.split_once("] [")?;
    let time = LogPrefixTime::parse(time_str)?; // "16:20:50"
    let (thread_level_context_str, rest_of_line) = rest.split_once("]: ")?;
    let (thread_level_str, context_str_opt) = if let Some((before, after)) = thread_level_context_str.rsplit_once("] [") {
        (before, Some(after))
    } else {
        (thread_level_context_str, None)
    };
    let (thread_str, level_str) = if let Some((thread, level)) = thread_level_str.split_once('/') {
        (thread, level)
    } else { // e.g. "[17:26:37] [WARN] [FabricLoader/Metadata]: ..."
        ("", thread_level_str) // No thread info, we use an empty string instead of none because this seems to be fairly rare
    };
    let prefix = LogPrefix {
        time,
        thread: thread_str.to_string(),
        level: level_str.to_string(),
        context: context_str_opt.map(|s| s.to_string()),
    };
    Some((prefix, rest_of_line))
}

// "2025-12-13 11:59:21 [INFO] [MiscPeripheralsASM] Initialized"
fn parse_partial_brackets(line: &str) -> Option<(LogPrefix, &str)> {
    let time_str = line.get(0..19)?; // "2025-12-13 11:59:21"
    let rest = line.get(19..)?; // " [INFO] [MiscPeripheralsASM] Initialized"
    let time = LogPrefixTime::parse(time_str)?;
    let (level, rest) = rest.strip_prefix(" [")?.split_once("] [")?;
    let (source, rest) = rest.split_once("] ")?;
    let prefix = LogPrefix {
        time,
        thread: source.to_string(),
        level: level.to_string(),
        context: None,
    };
    Some((prefix, rest))
}

// Seemingly quite rare
// "[17:23:00] [Client-Main] 24 Achievements"
fn parse_no_level(line: &str) -> Option<(LogPrefix, &str)> {
    let (time_str, rest) = line.strip_prefix('[')?.split_once("] [")?;
    let time = LogPrefixTime::parse(time_str)?;
    let (thread, rest) = rest.split_once("] ")?;
     let prefix = LogPrefix {
        time,
        thread: thread.to_string(),
        level: "".to_string(), // Not using None
        context: None,
    };
    Some((prefix, rest))
}

// "     0.001 D | Testing "/usr/share/PrismLauncher/qtlogging.ini" ..."
// "41.793 D | World Name: "New World""
fn parse_qtlogging(line: &str) -> Option<(LogPrefix, &str)> {
    let line = line.trim_ascii_start();
    let (sec_str, rest) = line.split_once('.')?;
    let sec: usize = sec_str.parse().ok()?;
    let (ms_str, rest) = rest.split_once(' ')?;
    let ms: u16 = ms_str.parse().ok()?;
    let (level, rest) = rest.split_once(" | ")?;
    let second: u8 = (sec % 60).try_into().ok()?;
    let minute: u8 = (sec / 60 % 60).try_into().ok()?;
    let hour: u8 = (sec / 60 / 60).try_into().ok()?;
    let time = LogPrefixTime {
        date: None,
        hour,
        minute,
        second,
        millisecond: Some(ms),
    };
    let prefix = LogPrefix { 
        time,
        thread: "".to_string(),
        level: level.to_string(),
        context: None,
    };
    Some((prefix, rest))
}

impl LogPrefix {
    /// Parses a log line prefix and returns the LogPrefix and the rest of the line if successful.
    pub fn parse(line: &str) -> Option<(LogPrefix, &str)> {
        let stripped = basic_strip_ansi_escape(line); // Get rid of ANSI color prefix

        parse_with_brackets(stripped)
            .or_else(|| parse_iso_no_brackets(stripped))
            .or_else(|| parse_partial_brackets(stripped))
            .or_else(|| parse_no_level(stripped))
            .or_else(|| parse_qtlogging(stripped))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let line = "[16:20:50] [Client thread/INFO]: LWJGL Version: 2.9.4";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix");
        assert_eq!(format!("{}", prefix.time), "16:20:50");
        assert_eq!(prefix.thread, "Client thread");
        assert_eq!(prefix.level, "INFO");
        assert_eq!(rest, "LWJGL Version: 2.9.4");
    }

    #[test]
    fn with_milliseconds() {
        let line = "[17:26:36.877] [main/INFO] [loading.moddiscovery.ModDiscoverer/SCAN]: Found mod file...";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix");
        assert_eq!(format!("{}", prefix.time), "17:26:36.877");
        assert_eq!(prefix.thread, "main");
        assert_eq!(prefix.level, "INFO");
        assert_eq!(prefix.context, Some("loading.moddiscovery.ModDiscoverer/SCAN".to_string()));
        assert_eq!(rest, "Found mod file...");
        let line2 = "[21:05:39] [main/WARN] [mixin/]: @Mixin target...";
        let (prefix2, rest2) = LogPrefix::parse(line2).expect("Failed to parse prefix with context");
        assert_eq!(format!("{}", prefix2.time), "21:05:39");
        assert_eq!(prefix2.thread, "main");
        assert_eq!(prefix2.level, "WARN");
        assert_eq!(prefix2.context, Some("mixin/".to_string()));
        assert_eq!(rest2, "@Mixin target...");
    }

    #[test]
    fn nested_brackets() {
        let line = "[21:08:14] [DH-Cleanup Thread[0]/INFO] [Di.se.di.co.re.LodQuadTree/]: waiting for [0] futures before closing render cache...";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with nested brackets");
        assert_eq!(format!("{}", prefix.time), "21:08:14");
        assert_eq!(prefix.thread, "DH-Cleanup Thread[0]");
        assert_eq!(prefix.level, "INFO");
        assert_eq!(prefix.context, Some("Di.se.di.co.re.LodQuadTree/".to_string()));
        assert_eq!(rest, "waiting for [0] futures before closing render cache...");
    }

    #[test]
    fn no_thread() {
        let line = "[17:26:37] [WARN] [FabricLoader/Metadata]: The mod \"betterstats\" contains invalid entries in its mod json:";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with no thread");
        assert_eq!(format!("{}", prefix.time), "17:26:37");
        assert_eq!(prefix.thread, "");
        assert_eq!(prefix.level, "WARN");
        assert_eq!(prefix.context, Some("FabricLoader/Metadata".to_string()));
        assert_eq!(rest, "The mod \"betterstats\" contains invalid entries in its mod json:");
    }

    #[test]
    fn ansi() {
        let line = "[m[32m[21:03:47.697] [main/INFO] [EARLYDISPLAY/]: Trying GL version 4.6";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with ANSI escape codes");
        assert_eq!(format!("{}", prefix.time), "21:03:47.697");
        assert_eq!(rest, "Trying GL version 4.6")
    }

    #[test]
    fn iso_no_brackets() {
        let line = "2025-10-30T19:21:06.036061Z main WARN Advanced terminal features are not available in this environment";
        let (prefix, _rest) = LogPrefix::parse(line).expect("Failed to parse prefix with no brackets and RFC3339 timestamp");
        assert_eq!(format!("{}", prefix.time), "2025-10-30 19:21:06.036");
    }

    #[test]
    fn test_without_level() {
        let line = "[17:23:00] [Client-Main] 24 Achievements";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix");
        assert_eq!(format!("{}", prefix.time), "17:23:00");
        assert_eq!(prefix.thread, "Client-Main");
        assert_eq!(rest, "24 Achievements");
    }

    #[test]
    fn test_parse_partial_brackets() {
        let line = "2024-07-11 04:30:53 [INFO] [ForgeModLoader] [AppEng] Core Init";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with partial brackets");
        assert_eq!(format!("{}", prefix.time), "2024-07-11 04:30:53");
        assert_eq!(prefix.level, "INFO");
        assert_eq!(prefix.thread, "ForgeModLoader");
        assert_eq!(rest, "[AppEng] Core Init");
    }

    #[test]
    fn test_parse_qtlogging() {
        let line = r#"     0.001 D | Testing "/usr/share/PrismLauncher/qtlogging.ini" ..."#;
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with partial brackets");
        assert_eq!(format!("{}", prefix.time), "00:00:00.001");
        assert_eq!(prefix.level, "D");
        assert_eq!(rest, r#"Testing "/usr/share/PrismLauncher/qtlogging.ini" ..."#);
    }
}
