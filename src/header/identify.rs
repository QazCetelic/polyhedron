#[derive(Debug, PartialEq)]
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

pub struct LauncherInfo {
    variant: LauncherVariant,
    version: String,
    distribution: Option<String>,
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

#[cfg(test)]
mod tests {
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
}
