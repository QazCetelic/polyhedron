use std::str::Lines;

#[derive(Debug)]
pub struct StacktraceParser {
    exception: Option<String>,
    message: Option<String>,
    lines: Vec<StacktraceLine>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct StacktraceLine {
    class: String,
    method: String,
    source: String,
    rest: String,
}

impl StacktraceLine {
    #[allow(dead_code)]
    pub fn get_jar(&self) -> Option<String> {
        // "~[lwjgl-2.9.4-nightly-20150209.jar:?]"
        let without_brackets = self.rest.strip_suffix(']')?.trim_start_matches('~').strip_prefix('[')?;
        // "lwjgl-2.9.4-nightly-20150209.jar:?"
        let (source, _) = without_brackets.split_once(':')?;
        if source.ends_with(".jar") {
            Some(source.to_string())
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Stacktrace {
    pub exception: Option<String>,
    pub message: Option<String>,
    lines: Vec<StacktraceLine>,
}

#[allow(dead_code)]
impl Stacktrace {
    pub fn from_lines(lines: Lines) -> impl Iterator<Item = Stacktrace> {
        let mut parser = StacktraceParser::new();
        // Throws in extra line, to indirectly call finalize
        lines.chain(vec![""]).filter_map(move |line| parser.parse_line(line))
    }
}

#[allow(dead_code)]
impl StacktraceParser {
    pub fn new() -> Self {
        StacktraceParser { 
            exception: None,
            message: None,
            lines: Vec::new()
        }
    }

    fn parse_trace_line(&mut self, line: &str) -> Option<StacktraceLine> {
        const FOUR_SPACES: &str = "    ";
        let without_space = line.strip_prefix("\t").or(line.strip_prefix(FOUR_SPACES))?;
        let without_at = without_space.strip_prefix("at ")?;
        let (class_and_method, source_and_rest) = without_at.split_once('(')?;
        let (class, method) = class_and_method.rsplit_once('.')?;
        let (source, rest) = source_and_rest.split_once(')')?;
        let rest = rest.trim_ascii_start();
        Some(StacktraceLine {
            class: class.to_string(),
            method: method.to_string(),
            source: source.to_string(),
            rest: rest.to_string(),
        })
    }

    pub fn parse_line(&mut self, line: &str) -> Option<Stacktrace> {
        if let Some((exception, message)) = line.split_once(": ") { // "org.lwjgl.LWJGLException: Could not choose GLX13 config"
            self.exception = Some(exception.to_string());
            self.message = Some(message.to_string());
            self.lines.clear();
            None
        }
        else if let Some(trace) = self.parse_trace_line(line) { // "   at org..."
            self.lines.push(trace);
            None
        }
        else { // Failed to parse line
            self.finalize()
        }
    }

    pub fn finalize(&mut self) -> Option<Stacktrace> {
        if self.lines.is_empty() {
            self.exception = None;
            self.message = None;
            None
        }
        else {
            let trace = Stacktrace {
                exception: self.exception.take(),
                message: self.message.take(),
                lines: std::mem::take(&mut self.lines),
            };
            self.lines = Vec::new();
            Some(trace)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_trace_line() {
        let line = "    at org.lwjgl.opengl.LinuxDisplayPeerInfo.initDefaultPeerInfo(Native Method)";
        let mut parser = StacktraceParser::new();
        let trace_line = parser.parse_trace_line(line).expect("Failed to parse");
        assert_eq!(trace_line.class, "org.lwjgl.opengl.LinuxDisplayPeerInfo");
        assert_eq!(trace_line.method, "initDefaultPeerInfo");
        assert_eq!(trace_line.source, "Native Method");
        assert_eq!(trace_line.rest, "");
    }

    #[test]
    fn parse_trace_line_with_jar() {
        let line = "\tat org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105) [NewLaunch.jar:?]";
        let mut parser = StacktraceParser::new();
        let trace_line = parser.parse_trace_line(line).expect("Failed to parse");
        assert_eq!(trace_line.rest, "[NewLaunch.jar:?]");
        assert_eq!(trace_line.get_jar().expect("Failed to get jar"), "NewLaunch.jar");
    }

    #[test]
    fn parse_trace_line_with_class() {
        let line = "\tat net.minecraftforge.client.ForgeHooksClient.createDisplay(ForgeHooksClient.java:327) ~[ForgeHooksClient.class:?]";
        let mut parser = StacktraceParser::new();
        let trace_line = parser.parse_trace_line(line).expect("Failed to parse");
        assert_eq!(trace_line.class, "net.minecraftforge.client.ForgeHooksClient");
    }

    #[test]
    fn parse_crash_report() {
        let crash_report = "---- Minecraft Crash Report ----
// Would you like a cupcake?

Time: 2025-09-16 22:58:36 CEST
Description: Initializing game

org.lwjgl.LWJGLException: Could not choose GLX13 config
    at org.lwjgl.opengl.LinuxDisplayPeerInfo.initDefaultPeerInfo(Native Method)
    at org.lwjgl.opengl.LinuxDisplayPeerInfo.<init>(LinuxDisplayPeerInfo.java:61)
    at org.lwjgl.opengl.LinuxDisplay.createPeerInfo(LinuxDisplay.java:828)
    at org.lwjgl.opengl.DrawableGL.setPixelFormat(DrawableGL.java:61)
    at org.lwjgl.opengl.Display.create(Display.java:846)
    at org.lwjgl.opengl.Display.create(Display.java:757)
    at org.lwjgl.opengl.Display.create(Display.java:739)
    at net.minecraft.client.Minecraft.func_71384_a(Minecraft.java:452)
    at net.minecraft.client.Minecraft.func_99999_d(Minecraft.java:7099)
    at net.minecraft.client.main.Main.main(SourceFile:148)
    at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
    at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)
    at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
    at java.lang.reflect.Method.invoke(Method.java:498)
    at net.minecraft.launchwrapper.Launch.launch(Launch.java:135)
    at net.minecraft.launchwrapper.Launch.main(Launch.java:28)
    at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
    at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
    at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
    ";
        let lines = crash_report.lines();
        let mut traces = Stacktrace::from_lines(lines);
        let trace = traces.next().expect("Failed to get trace");
        assert!(traces.next().is_none());

        assert_eq!(trace.exception.expect("No exception"), "org.lwjgl.LWJGLException");
        assert_eq!(trace.message.expect("No message"), "Could not choose GLX13 config");
        assert_eq!(trace.lines.len(), 19);
        let first_trace_line = trace.lines.first().expect("No trace lines");
        assert_eq!(first_trace_line.method, "initDefaultPeerInfo");
        let last_trace_line = trace.lines.last().expect("No trace lines");
        assert_eq!(last_trace_line.source, "EntryPoint.java:70");
    }
}