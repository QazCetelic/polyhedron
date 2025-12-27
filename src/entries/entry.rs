use crate::entries::prefix::LogPrefix;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct LogEntry {
    pub prefix: LogPrefix,
    pub contents: String,
}