#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum LauncherVariant {
    PrismLauncher,
    MultiMC,
    Other(String), // Other MultiMC / Prism forks
}

impl LauncherVariant {
    fn from_str(s: &str) -> Self {
        match s {
            "Prism Launcher" => LauncherVariant::PrismLauncher,
            "MultiMC" => LauncherVariant::MultiMC,
            other => LauncherVariant::Other(other.to_string()),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LauncherInfo {
    pub variant: LauncherVariant,
    pub version: String,
    pub distribution: Option<String>,
}

#[allow(dead_code)]
impl LauncherInfo {
    pub fn from_first_line(log: &str) -> Option<Self> {
        // e.g. "Prism Launcher version: 9.4 (official)", "MultiMC version: 0.7.0-4230", "Prism Launcher version: 9.4 (flatpak)"
        let first_line = log.lines().next()?;
        let (variant_str, variant_and_distribution) = first_line.split_once(" version: ")?;
        let variant = LauncherVariant::from_str(variant_str);
        let (version, distribution) = if let Some(pos) = variant_and_distribution.find(" (") {
            let version = variant_and_distribution.get(..pos)?;
            let distribution = &variant_and_distribution.get(pos + 2..variant_and_distribution.len() - 1)?;
            (version.to_string(), Some(distribution.to_string()))
        } else {
            (variant_and_distribution.to_string(), None)
        };
        Some(LauncherInfo {
            variant,
            version,
            distribution,
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct LauncherVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub rest: String,
}

impl LauncherVersion {
    pub fn parse(version: &str) -> Option<Self> {
        let (ver, rest) = version.split_once('-').unwrap_or_else(|| (version, ""));
        let (major_str, minor_and_patch) = ver.split_once('.')?;
        let major: u32 = major_str.parse().ok()?;
        let (minor_str, patch_str) = minor_and_patch.split_once('.').unwrap_or_else(|| (minor_and_patch, ""));
        let minor: u32 = minor_str.parse().ok()?;
        let patch: u32 = if patch_str != "" { patch_str.parse::<u32>().ok()? } else { 0 };

        Some(LauncherVersion { major, minor, patch, rest: rest.to_string() })
    }
}

#[cfg(test)]
mod tests {
    use crate::header::identify::LauncherVersion;

    #[test]
    fn test_get_launcher_info() {
        let log1 = "Prism Launcher version: 9.4 (official)\nSome other log content...";
        let info1 = super::LauncherInfo::from_first_line(log1).unwrap();
        assert_eq!(info1.variant, super::LauncherVariant::PrismLauncher);
        assert_eq!(info1.version, "9.4");
        assert_eq!(info1.distribution.as_deref(), Some("official"));

        let log2 = "MultiMC version: 0.7.0-4230\nSome other log content...";
        let info2 = super::LauncherInfo::from_first_line(log2).unwrap();
        assert_eq!(info2.variant, super::LauncherVariant::MultiMC);
        assert_eq!(info2.version, "0.7.0-4230");
        assert_eq!(info2.distribution, None);

        let log3 = "Prism Launcher version: 9.4 (flatpak)\nSome other log content...";
        let info3 = super::LauncherInfo::from_first_line(log3).unwrap();
        assert_eq!(info3.variant, super::LauncherVariant::PrismLauncher);
        assert_eq!(info3.version, "9.4");
        assert_eq!(info3.distribution.as_deref(), Some("flatpak"));
    }

    #[test]
    fn parse_version() {
        let version1_str = "9.2";
        let version1 = LauncherVersion::parse(version1_str).expect("Failed to parse");
        assert_eq!((version1.major, version1.minor), (9, 2));
        let version2_str = "10.0.0-develop";
        let version2 = LauncherVersion::parse(version2_str).expect("Failed to parse");
        assert_eq!((version2.major, version2.minor), (10, 0));
        assert!(version2 > version1);
    }
}
