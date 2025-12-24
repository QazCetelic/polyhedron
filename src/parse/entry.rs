use crate::parse::prefix::LogPrefix;

#[derive(Debug)]
pub struct LogEntry {
    pub prefix: LogPrefix,
    pub contents: String,
}