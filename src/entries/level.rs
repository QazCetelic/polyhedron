pub enum LogLevel {
    INFO,
    WARNING,
    ERROR,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Option<LogLevel> {
        if s.eq_ignore_ascii_case("INFO") {
            Some(LogLevel::INFO)
        }
        else if s.eq_ignore_ascii_case("WARN") {
            Some(LogLevel::WARNING)
        }
        else if s.eq_ignore_ascii_case("WARNING") {
            Some(LogLevel::WARNING)
        }
        else if s.eq_ignore_ascii_case("ERROR") {
            Some(LogLevel::ERROR)
        }
        else {
            None
        }
    }
}