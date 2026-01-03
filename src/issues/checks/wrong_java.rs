use lazy_regex::regex;

use crate::issues::issue::Issue;

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
}