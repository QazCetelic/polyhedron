use crate::{header::{identify::{LauncherInfo, LauncherVersion}, index::IndexedLogHeader}, issues::issue::Issue};

fn outdated_launcher(launcher_info: &LauncherInfo, latest_version: &str) -> Option<Issue> {
    let used_version = LauncherVersion::parse(&launcher_info.version)?;
    let latest_version = LauncherVersion::parse(latest_version)?;
    (used_version < latest_version).then_some(Issue::OutdatedLauncher)
}

pub(crate) fn outdated_launcher_header(header: &IndexedLogHeader) -> Option<Issue> {
    const LAST_KNOWN_VERSION: &str = "10.0.0"; // I don't want it to depend on network requests so we just hardcode it
    let launcher_info = LauncherInfo::from_first_line(header.text)?;
    outdated_launcher(&launcher_info, LAST_KNOWN_VERSION)
}

#[cfg(test)]
mod tests {
    use crate::header::identify::LauncherInfo;

    use super::*;

    #[test]
    fn matches_outdated_launcher() {
        let header_fragment = "Prism Launcher version: 9.3 (archlinux)";
        let launcher_info = LauncherInfo::from_first_line(header_fragment).expect("Failed to extract launcher info");
        let issue = outdated_launcher(&launcher_info, "9.4").expect("Failed to determine issue");
        assert_eq!(issue, Issue::OutdatedLauncher);
    }

    #[test]
    fn matches_up_to_date_launcher() {
        let header_fragment = "Prism Launcher version: 9.4 (archlinux)";
        let launcher_info = LauncherInfo::from_first_line(header_fragment).expect("Failed to extract launcher info");
        let issue = outdated_launcher(&launcher_info, "9.4");
        assert_eq!(issue, None);
    }
}