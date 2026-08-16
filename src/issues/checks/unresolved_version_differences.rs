use crate::{entries::entry::LogEntry, issues::issue::Issue};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnresolvedVersionDifferences {
    pub mod_name: String,
    pub version_1: String,
    pub version_2: String,
}

pub(crate) fn unresolved_version_differences(entry: &LogEntry) -> Option<Issue> {
    let text = entry.contents.strip_prefix("The following mods have version differences that were not resolved:")?.trim_ascii_start();
    let mut version_differences = Vec::new();
    for line in text.lines() {
        if line.starts_with("Things may not work well.") {
            return Some(Issue::UnresolvedVersionDifferences(version_differences));
        }
        let (mod_name, versions) = line.split_once(" (version ")?;
        let (version_1, version_2) = versions.strip_suffix(")").and_then(|v| v.split_once(" -> "))?;
        let version_difference = UnresolvedVersionDifferences {
            mod_name: mod_name.to_string(),
            version_1: version_1.to_string(),
            version_2: version_2.to_string(),
        };
        version_differences.push(version_difference);
    }
    Some(Issue::UnresolvedVersionDifferences(version_differences))
}


#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn test_1() {
        let text = "[11Oct2025 18:57:38.621] [main/DEBUG] [mixin/]: Mixing common.DirectoryLockMixin from aether.mixins.json into net.minecraft.util.DirectoryLock
[11Oct2025 18:57:38.632] [main/DEBUG] [mixin/]: Mixing accessor.NbtAccounterAccessor from create.mixins.json into net.minecraft.nbt.NbtAccounter
[11Oct2025 18:57:38.792] [main/WARN] [net.minecraftforge.common.ForgeHooks/WP]: The following mods have version differences that were not resolved:
twilightforest (version 4.3.2508 -> MISSING)
Things may not work well.";
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| unresolved_version_differences(e)).next().expect("Failed to determine issue");
        let version_differences = vec![
            UnresolvedVersionDifferences {
                mod_name: "twilightforest".to_string(),
                version_1: "4.3.2508".to_string(),
                version_2: "MISSING".to_string(),
            }
        ];
        assert_eq!(issue, Issue::UnresolvedVersionDifferences(version_differences));
    }

    #[test]
    fn test_2() {
        let text = "[21:39:21] [Render thread/ERROR] [EMF/]: [EMF]: model attempted creation more than 64 times {minecraft:player#cape]. EMF is now ignoring this model. Please inform the mod maker that this is not how entity models are meant to be utilised. They should ALWAYS be stored and reused.
[21:39:23] [Render thread/WARN] [ne.ne.ne.co.CommonHooks/WP]: The following mods have version differences that were not resolved:
drippyloadingscreen (version 3.1.0 -> MISSING)
fancymenu (version 3.8.1 -> MISSING)
konkrete (version 1.9.9 -> MISSING)
melody (version 1.0.10 -> MISSING)
Things may not work well.
[21:39:23] [Render thread/ERROR] [minecraft/AbstractPackResources]: Couldn't load fabric:overlays metadata
";
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| unresolved_version_differences(e)).next().expect("Failed to determine issue");
        let version_differences = vec![
            UnresolvedVersionDifferences {
                mod_name: "drippyloadingscreen".to_string(),
                version_1: "3.1.0".to_string(),
                version_2: "MISSING".to_string(),
            },
            UnresolvedVersionDifferences {
                mod_name: "fancymenu".to_string(),
                version_1: "3.8.1".to_string(),
                version_2: "MISSING".to_string(),
            },
            UnresolvedVersionDifferences {
                mod_name: "konkrete".to_string(),
                version_1: "1.9.9".to_string(),
                version_2: "MISSING".to_string(),
            },
            UnresolvedVersionDifferences {
                mod_name: "melody".to_string(),
                version_1: "1.0.10".to_string(),
                version_2: "MISSING".to_string(),
            },
        ];
        assert_eq!(issue, Issue::UnresolvedVersionDifferences(version_differences));
    }

    #[test]
    fn test_3() {
        let text = "[11:23:21] [Render thread/INFO] [chloride/]: Registering CHLORIDE built-in packs
[11:23:21] [Render thread/WARN] [ne.mi.co.ForgeHooks/WP]: The following mods have version differences that were not resolved:
entity_model_features (version 3.0.7 -> 3.0.12)
entity_texture_features (version 7.0.8 -> 7.0.9)
Things may not work well.
[11:23:21] [Render thread/WARN] [ne.mi.re.ForgeRegistry/REGISTRIES]: Registry minecraft:block: Object did not get ID it asked for. Name: immersiveenchanting:creative_bookshelf Expected: 1005 Got: 4338
";
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| unresolved_version_differences(e)).next().expect("Failed to determine issue");
        let version_differences = vec![
            UnresolvedVersionDifferences {
                mod_name: "entity_model_features".to_string(),
                version_1: "3.0.7".to_string(),
                version_2: "3.0.12".to_string(),
            },
            UnresolvedVersionDifferences {
                mod_name: "entity_texture_features".to_string(),
                version_1: "7.0.8".to_string(),
                version_2: "7.0.9".to_string(),
            },
        ];
        assert_eq!(issue, Issue::UnresolvedVersionDifferences(version_differences));
    }
}