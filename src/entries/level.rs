#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum LogLevel {
    INFO,
    WARNING,
    ERROR,
    FATAL,
}

pub const LOG_LEVELS: [LogLevel; 4] = [LogLevel::INFO, LogLevel::WARNING, LogLevel::ERROR, LogLevel::FATAL];

impl LogLevel {
    pub fn from_str(s: &str) -> Option<LogLevel> {
        if s.eq_ignore_ascii_case("INFO") {
            Some(LogLevel::INFO)
        }
        else if s.eq_ignore_ascii_case("WARN") || s.eq_ignore_ascii_case("WARNING") {
            Some(LogLevel::WARNING)
        }
        else if s.eq_ignore_ascii_case("ERROR") {
            Some(LogLevel::ERROR)
        }
        else if s.eq_ignore_ascii_case("FATAL") || s.eq_ignore_ascii_case("CRITICAL") {
            Some(LogLevel::FATAL)
        }
        else {
            None
        }
    }
}