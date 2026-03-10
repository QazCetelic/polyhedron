use lazy_regex::regex;

use crate::{entries::entry::LogEntry, header::index::IndexedLogHeader, issues::issue::Issue};

fn wrong_java(text: &str) -> Option<Issue> {
    let switch_version_regex = regex!(r"Please switch to one of the following Java versions for this instance:[\r\n]+(Java version [\d.]+)");

    if let Some(captures) = switch_version_regex.captures(text) {
		let version = (&captures[1]).strip_prefix("Java version ")?.parse::<u32>().ok()?;
		Some(Issue::WrongJava(Some(version)))
	}
    else if text.contains("Java major version is incompatible. Things might break.") {
        Some(Issue::WrongJava(None))
    }
    else {
        None
    }
}

pub(crate) fn wrong_java_header(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    wrong_java(header.text)
}

pub(crate) fn class_file_version_not_supported(entry: &LogEntry) -> Option<Issue> {
    let (_, etc) = entry.contents.split_once("Class file major version ")?;
    let (_class_file_ver_used, jvm_ver_etc) = etc.split_once(" is not supported by active ASM (version ")?;
    let (_used_jvm_ver, class_ver_etc) = jvm_ver_etc.split_once(" supports class version ")?;
    let (_class_file_ver_supported, _) = class_ver_etc.split_once(")")?;
    Some(Issue::WrongJava(None))
}

pub(crate) fn argument_exception_class_file_version(entry: &LogEntry) -> Option<Issue> {
    (entry.contents.contains("java.lang.IllegalArgumentException: Unsupported class file major version ")).then_some(Issue::WrongJava(None))
}

pub(crate) fn unsupported_class_version(entry: &LogEntry) -> Option<Issue> {
    (entry.contents.contains("java.lang.UnsupportedClassVersionError")).then_some(Issue::WrongJava(None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn please_switch_version() {
        let header_fragment = "This instance is not compatible with Java version 16.
Please switch to one of the following Java versions for this instance:
Java version 8
Go to instance Java settings to change your Java version or disable the Java compatibility check if you know what you're doing.
";
        let issue = wrong_java(&header_fragment).expect("Failed to determine issue");
        assert_eq!(issue, Issue::WrongJava(Some(8)));
    }

    #[test]
    fn incompatible_version_warning() {
        let header_fragment = "
Window size: 854 x 480

Launcher: standard

Java major version is incompatible. Things might break.
Java Arguments:
[-XX: UnlockExperimentalVMOptions, -XX: UseZGC, -XX: ZGenerational, -XX: AlwaysPreTouch, -Xms6144m, -Xmx8192m, -Duser.language=en]


Minecraft process ID: 5295";
        let issue = wrong_java(&header_fragment).expect("Failed to determine issue");
        assert_eq!(issue, Issue::WrongJava(None));
    }

    #[test]
    fn class_file_version_with_version() {
        let log = r#"[02:53:35] [main/WARN] [mixin]: Error loading class: java/lang/invoke/MethodHandles$Lookup (java.lang.IllegalArgumentException: Class file major version 69 is not supported by active ASM (version 9.0 supports class version 68), reading java/lang/invoke/MethodHandles$Lookup)"#;
        let entries = LogEntry::from_lines(log.lines());
        let entry = entries.first().expect("Failed to get entry");
        let issue = class_file_version_not_supported(entry).expect("Failed to determine issue");
        assert_eq!(issue, Issue::WrongJava(None));
    }

    #[test]
    fn argument_exception_unsupported_class_file() {
        let log = r#"[12:36:01] [Client thread/WARN] [mixin]: Error loading class: gravisuite/ItemAdvancedLappack (java.lang.IllegalArgumentException: Unsupported class file major version 27969)"#;
        let entries = LogEntry::from_lines(log.lines());
        let entry = entries.first().expect("Failed to get entry");
        let issue = argument_exception_class_file_version(entry).expect("Failed to determine issue");
        assert_eq!(issue, Issue::WrongJava(None));
    }

    #[test]
    fn unsupported_class_version_thrown() {
        let log = r#"[15:09:26] [main/ERROR]: Minecraft has crashed!
net.fabricmc.loader.impl.FormattedException: java.lang.UnsupportedClassVersionError: net/minecraft/class_425 has been compiled by a more recent version of the Java Runtime (class file version 65.0), this version of the Java Runtime only recognizes class file versions up to 61.0
	at net.fabricmc.loader.impl.FormattedException.ofLocalized(FormattedException.java:63) ~[fabric-loader-0.18.4.jar:?]
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:516) ~[fabric-loader-0.18.4.jar:?]
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72) ~[fabric-loader-0.18.4.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23) ~[fabric-loader-0.18.4.jar:?]
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:115) ~[NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129) ~[NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70) ~[NewLaunch.jar:?]"#;
        let entries = LogEntry::from_lines(log.lines());
        let entry = entries.first().expect("Failed to get entry");
        let issue = unsupported_class_version(entry).expect("Failed to determine issue");
        assert_eq!(issue, Issue::WrongJava(None));
    }
}