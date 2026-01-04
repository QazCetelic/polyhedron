use crate::entries::time::LogPrefixTime;


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LogPrefix {
    /// Timestamp of the log entry, optionally including date and milliseconds
    pub time: LogPrefixTime,
    /// Thread name from which the log originated
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

impl LogPrefix {
    /// Parses a log line prefix and returns the LogPrefix and the rest of the line if successful.
    pub fn parse(line: &str) -> Option<(LogPrefix, &str)> {
        let stripped = basic_strip_ansi_escape(line); // Get rid of ANSI color prefix

        // "[16:20:50] [Client thread/INFO]: LWJGL Version: 2.9.4"
        // "[17:26:36.877] [main/INFO] [loading.moddiscovery.ModDiscoverer/SCAN]: Found mod file..."
        if let Some(stripped) = stripped.strip_prefix('[') {
            let (time_str, rest) = stripped.split_once("] [")?;
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
        // "2025-10-30T19:21:06.036061Z main WARN Advanced terminal features are not available in this environment"
        else {
            let (time_str, rest) = stripped.split_once(' ')?;
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_prefix() {
        let line = "[16:20:50] [Client thread/INFO]: LWJGL Version: 2.9.4";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix");
        assert_eq!(prefix.time.hour, 16);
        assert_eq!(prefix.time.minute, 20);
        assert_eq!(prefix.time.second, 50);
        assert_eq!(prefix.thread, "Client thread");
        assert_eq!(prefix.level, "INFO");
        assert_eq!(rest, "LWJGL Version: 2.9.4");
    }

    #[test]
    fn test_parse_with_context() {
        let line = "[17:26:36.877] [main/INFO] [loading.moddiscovery.ModDiscoverer/SCAN]: Found mod file...";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix");
        assert_eq!(prefix.time.hour, 17);
        assert_eq!(prefix.time.minute, 26);
        assert_eq!(prefix.time.second, 36);
        assert_eq!(prefix.time.millisecond, Some(877));
        assert_eq!(prefix.thread, "main");
        assert_eq!(prefix.level, "INFO");
        assert_eq!(prefix.context, Some("loading.moddiscovery.ModDiscoverer/SCAN".to_string()));
        assert_eq!(rest, "Found mod file...");
        let line2 = "[21:05:39] [main/WARN] [mixin/]: @Mixin target...";
        let (prefix2, rest2) = LogPrefix::parse(line2).expect("Failed to parse prefix with context");
        assert_eq!(prefix2.time.hour, 21);
        assert_eq!(prefix2.time.minute, 5);
        assert_eq!(prefix2.time.second, 39);
        assert_eq!(prefix2.thread, "main");
        assert_eq!(prefix2.level, "WARN");
        assert_eq!(prefix2.context, Some("mixin/".to_string()));
        assert_eq!(rest2, "@Mixin target...");
    }

    #[test]
    fn test_parse_nested_brackets() {
        let line = "[21:08:14] [DH-Cleanup Thread[0]/INFO] [Di.se.di.co.re.LodQuadTree/]: waiting for [0] futures before closing render cache...";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with nested brackets");
        assert_eq!(prefix.time.hour, 21);
        assert_eq!(prefix.time.minute, 8);
        assert_eq!(prefix.time.second, 14);
        assert_eq!(prefix.thread, "DH-Cleanup Thread[0]");
        assert_eq!(prefix.level, "INFO");
        assert_eq!(prefix.context, Some("Di.se.di.co.re.LodQuadTree/".to_string()));
        assert_eq!(rest, "waiting for [0] futures before closing render cache...");
    }

    #[test]
    fn test_parse_no_thread() {
        let line = "[17:26:37] [WARN] [FabricLoader/Metadata]: The mod \"betterstats\" contains invalid entries in its mod json:";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with no thread");
        assert_eq!(prefix.time.hour, 17);
        assert_eq!(prefix.time.minute, 26);
        assert_eq!(prefix.time.second, 37);
        assert_eq!(prefix.thread, "");
        assert_eq!(prefix.level, "WARN");
        assert_eq!(prefix.context, Some("FabricLoader/Metadata".to_string()));
        assert_eq!(rest, "The mod \"betterstats\" contains invalid entries in its mod json:");
    }

    #[test]
    fn test_strip_ansi() {
        let line = "[m[32m[21:03:47.697] [main/INFO] [EARLYDISPLAY/]: Trying GL version 4.6";
        let stripped_line = basic_strip_ansi_escape(line);
        assert_eq!(stripped_line, "[21:03:47.697] [main/INFO] [EARLYDISPLAY/]: Trying GL version 4.6");
    }

    #[test]
    fn test_parse_ansi() {
        let line = "[m[32m[21:03:47.697] [main/INFO] [EARLYDISPLAY/]: Trying GL version 4.6";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with ANSI escape codes");
        assert_eq!(prefix.time.hour, 21);
    }

    #[test]
    fn test_parse_no_brackets() {
        let line = "2025-10-30T19:21:06.036061Z main WARN Advanced terminal features are not available in this environment";
        let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with no brackets and RFC3339 timestamp");
    }

    // TODO
    // #[test]
    // fn test_parse_partial_brackets() {
    //     let line = "2024-07-11 04:30:53 [INFO] [ForgeModLoader] [AppEng] Core Init";
    //     let (prefix, rest) = LogPrefix::parse(line).expect("Failed to parse prefix with partial brackets");
    // }
}
