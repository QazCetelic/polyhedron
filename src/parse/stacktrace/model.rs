use crate::parse::stacktrace::parse::valid_java_identifier;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dioxius", derive(PartialEq))]
#[derive(Clone)]
pub struct Stacktrace {
    pub exception: String,
    pub message: String,
    pub lines: Vec<StacktraceLine>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct StacktraceLine {
    pub class: String,
    pub method: String,
    pub source: String,
    pub source_info: String,
}

impl StacktraceLine {
    #[allow(dead_code)]
    pub fn get_jar(&self) -> Option<String> {
        // "~[lwjgl-2.9.4-nightly-20150209.jar:?]"
        let without_brackets = self.source_info.strip_suffix(']')?.trim_start_matches('~').strip_prefix('[')?;
        // "lwjgl-2.9.4-nightly-20150209.jar:?"
        let (source, _) = without_brackets.split_once(':')?;
        if source.ends_with(".jar") {
            Some(source.to_string())
        }
        else {
            None
        }
    }

    /// Gets the relative file path of the source code and the line
    pub fn get_relative_path(&self) -> Option<(String, usize)> {
        // "EntryPoint.java:70"
        let (class_name, line) = self.source.split_once(".java:")?;
        let line: usize = line.parse().ok()?;
        if !valid_java_identifier(class_name) || !self.class.ends_with(class_name) {
            return None;
        }
        let mut path = String::with_capacity(self.class.len());
        let mut iter = self.class.split('.').peekable();
        while let Some(part) = iter.next() {
            path.push_str(part);
            if iter.peek().is_some() {
                path.push('/');
            }
        }
        path.push_str(".java");
        Some((path, line))
    }
}