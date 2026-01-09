use lazy_regex::regex;

use crate::{header::index::IndexedLogHeader, issues::issue::Issue};

pub(crate) fn java_option(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    let vm_option_regex = regex!(r"Unrecognized VM option '(.+)'[\r\n]");
    let unrecognized_option_regex = regex!(r"Unrecognized option: (.+)[\r\n]");

    if let Some(captures) = vm_option_regex.captures(header.text) {
        let arg = format!("-XX:{}", &captures[1]);
        dbg!(&arg);
        return Some(Issue::JavaOption(arg));
	}
	else if let Some(captures) = unrecognized_option_regex.captures(header.text) {
        let arg = captures[1].to_string();
        return Some(Issue::JavaOption(arg));
	}

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_option_shenandoah() {
        let header_fragment = "Unrecognized VM option 'UseShenandoahGC'\n";
        let indexed = IndexedLogHeader::index_header(header_fragment);
        let issue = java_option(&indexed).expect("Failed to determine issue");
        assert_eq!(issue, Issue::JavaOption("-XX:UseShenandoahGC".to_string()));
    }

    #[test]
    fn vm_option_zgc() {
        let header_fragment = r#"--username  --version 1.16.1 --gameDir C:/Users/REDACTED/AppData/Roaming/PrismLauncher/instances/1.16.1/minecraft --assetsDir C:/Users/REDACTED/AppData/Roaming/PrismLauncher/assets --assetIndex 1.16 --uuid  --accessToken  --userType  --versionType release

Window size: 854 x 480

Launcher: standard

Java Arguments:
[-XX:+UseZGC, -XX:+AlwaysPreTouch, -Djdk.graal.TuneInlinerExploration=1, -XX:NmethodSweepActivity=1, -XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump, -Xms512m, -Xmx8500m, -Duser.language=en]


Minecraft process ID: 17292


Unrecognized VM option 'UseZGC'
Process exited with code 1.
"#;
        let indexed = IndexedLogHeader::index_header(header_fragment);
        let issue = java_option(&indexed).expect("Failed to determine issue");
        assert_eq!(issue, Issue::JavaOption("-XX:UseZGC".to_string()));
    }
}