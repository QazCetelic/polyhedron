use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub(crate) fn shader_compile_error(entry: &LogEntry) -> Option<Issue> {
    let found = entry.contents.starts_with("OpenGL debug message:") && entry.contents.contains("GLSL compile failed");
    found.then_some(Issue::ShaderCompileError)
}