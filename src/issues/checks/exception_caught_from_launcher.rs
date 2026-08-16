use crate::{entries::entry::LogEntry, header::index::IndexedLogHeader, issues::issue::Issue, parse::stacktrace::model::Stacktrace};

fn exception_caught_from_launcher(text: &str) -> Option<Issue> {
    let (_, str) = text.split_once("Exception caught from launcher")?;
    let (str, _) = str.split_once("Exiting with ERROR")?;
    let stacktraces = Stacktrace::from_lines(str.lines()).collect();
    Some(Issue::ExceptionCaughtFromLauncher(stacktraces))
}

pub(crate) fn exception_caught_from_launcher_header(header: &IndexedLogHeader<'_>) -> Option<Issue> {
	exception_caught_from_launcher(header.text)
}

pub(crate) fn exception_caught_from_launcher_entry(entry: &LogEntry) -> Option<Issue> {
	exception_caught_from_launcher(&entry.contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let header_fragement = "Minecraft process ID: 15364


Exception caught from launcher
java.lang.UnsupportedOperationException: Could not detect the libraries folder - it can be manually specified with `-Dforgewrapper.librariesDir=` (Java runtime argument)
	at io.github.zekerzhayard.forgewrapper.installer.detector.IFileDetector.getLibraryDir(IFileDetector.java:64)
	at io.github.zekerzhayard.forgewrapper.installer.detector.MultiMCFileDetector.getLibraryDir(MultiMCFileDetector.java:24)
	at io.github.zekerzhayard.forgewrapper.installer.detector.MultiMCFileDetector.getInstallerJar(MultiMCFileDetector.java:34)
	at io.github.zekerzhayard.forgewrapper.installer.Main.main(Main.java:36)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
Exiting with ERROR
Process exited with code 2.
Log upload triggered at: 12 Oct 2025 08:29:39  0200";
        let issue = exception_caught_from_launcher(&header_fragement).expect("Failed to determine issue");
        let Issue::ExceptionCaughtFromLauncher(stacktraces) = issue else {
            panic!("Expected ExceptionCaughtFromLauncher issue");
        };
        assert_eq!(stacktraces.len(), 1);
        let stacktrace = &stacktraces[0];
        assert_eq!(stacktrace.exception, "java.lang.UnsupportedOperationException");
        assert_eq!(stacktrace.message, "Could not detect the libraries folder - it can be manually specified with `-Dforgewrapper.librariesDir=` (Java runtime argument)");
    }

    #[test]
    fn test_2() {
        let header_fragement = "
Minecraft process ID: 29448


Exception caught from launcher
java.lang.NoClassDefFoundError: cpw/mods/modlauncher/Launcher
	at io.github.zekerzhayard.forgewrapper.installer.detector.IFileDetector.getLibraryDir(IFileDetector.java:32)
	at io.github.zekerzhayard.forgewrapper.installer.detector.MultiMCFileDetector.getLibraryDir(MultiMCFileDetector.java:24)
	at io.github.zekerzhayard.forgewrapper.installer.detector.MultiMCFileDetector.getInstallerJar(MultiMCFileDetector.java:34)
	at io.github.zekerzhayard.forgewrapper.installer.Main.main(Main.java:37)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
Caused by: java.lang.ClassNotFoundException: cpw.mods.modlauncher.Launcher
	at java.base/jdk.internal.loader.BuiltinClassLoader.loadClass(BuiltinClassLoader.java:641)
	at java.base/jdk.internal.loader.ClassLoaders$AppClassLoader.loadClass(ClassLoaders.java:188)
	at java.base/java.lang.ClassLoader.loadClass(ClassLoader.java:526)
	... 7 more
Exiting with ERROR
Process exited with code 2.
Log upload triggered at: 01 Oct 2025 22:03:08 -0700";
        let issue = exception_caught_from_launcher(&header_fragement).expect("Failed to determine issue");
        let Issue::ExceptionCaughtFromLauncher(stacktraces) = issue else {
            panic!("Expected ExceptionCaughtFromLauncher issue");
        };
        assert_eq!(stacktraces.len(), 2);
        let stacktrace = &stacktraces[0];
        assert_eq!(stacktrace.exception, "java.lang.NoClassDefFoundError");
        assert_eq!(stacktrace.message, "cpw/mods/modlauncher/Launcher");
    }
}