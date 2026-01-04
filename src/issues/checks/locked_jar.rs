use crate::issues::issue::Issue;

fn locked_jar(header_text: &str) -> Option<Issue> {
    const EXTRACT_FAIL_PREFIX: &str = "Couldn't extract native jar '";
    let mut index: usize = 0;
    let mut jars = Vec::new();
    while let Some(extract_fail_index) = header_text.get(index..).map(|s| s.find(EXTRACT_FAIL_PREFIX).map(|i| i + index)).flatten() {
        let line = header_text.get(extract_fail_index..)?.lines().next()?;
        let (jar, _destination) = line.strip_prefix(EXTRACT_FAIL_PREFIX)?.strip_suffix('\'')?.split_once("' to destination '")?;
        jars.push(jar.to_string());

        index = extract_fail_index + EXTRACT_FAIL_PREFIX.len();
    }

    Some(Issue::LockedJars(jars))
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::issues::issue::Issue;

    use super::*;

    #[test]
    fn find_in_header() {
        let header_fragment = r"Window size: 854 x 480

Launcher: standard

Couldn't extract native jar 'C:/Users/REDACTED/AppData/Roaming/PrismLauncher/libraries/com/mojang/text2speech/1.12.4/text2speech-1.12.4-natives-windows.jar' to destination 'C:/Users/REDACTED/AppData/Roaming/PrismLauncher/instances/Chroma Endless 2-1.1.3/natives'
Clipboard copy at: 11 Jan 2025 12:37:41 +0200
";

        let issue = locked_jar(&header_fragment).expect("Failed to determine issue");
        // assert_eq!(issue, Issue::LockedJars);
    }

    #[test]
    fn find_multiple() {
        let text = r"Couldn't extract native jar 'C:/Users/alhos/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl/lwjgl-platform/2.9.4-nightly-20150209/lwjgl-platform-2.9.4-nightly-20150209-natives-windows.jar' to destination 'C:/Users/alhos/AppData/Roaming/PrismLauncher/instances/Hypixel/natives'
Couldn't extract native jar 'C:/Users/alhos/AppData/Roaming/PrismLauncher/libraries/tv/twitch/twitch-platform/6.5/twitch-platform-6.5-natives-windows-64.jar' to destination 'C:/Users/alhos/AppData/Roaming/PrismLauncher/instances/Hypixel/natives'
Couldn't extract native jar 'C:/Users/alhos/AppData/Roaming/PrismLauncher/libraries/tv/twitch/twitch-external-platform/4.5/twitch-external-platform-4.5-natives-windows-64.jar' to destination 'C:/Users/alhos/AppData/Roaming/PrismLauncher/instances/Hypixel/natives'";
        let issue = locked_jar(&text).expect("Failed to determine issue");
        let Issue::LockedJars(locked_jars) = issue else { panic!("Not LockedJars issue"); };
        assert_eq!(locked_jars, vec![
            "C:/Users/alhos/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl/lwjgl-platform/2.9.4-nightly-20150209/lwjgl-platform-2.9.4-nightly-20150209-natives-windows.jar",
            "C:/Users/alhos/AppData/Roaming/PrismLauncher/libraries/tv/twitch/twitch-platform/6.5/twitch-platform-6.5-natives-windows-64.jar",
            "C:/Users/alhos/AppData/Roaming/PrismLauncher/libraries/tv/twitch/twitch-external-platform/4.5/twitch-external-platform-4.5-natives-windows-64.jar"
        ]);
    }
}