use crate::{entries::entry::LogEntry, issues::issue::Issue};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModVersionConflictInfo {
    pub mod_name: String,
    pub normalized_mod_name: String,
    pub message: String,
    pub action: String,
}

fn parse_mod_line(mod_line: &str) -> Result<(String, String, String), &'static str> {
    let (_, rest) = mod_line.split_once("- Mod '").ok_or("mod prefix missing")?;
    let (mod_name, rest) = rest.split_once("' (").ok_or("mod name missing")?;
    let (mod_id, rest) = rest.split_once(") ").ok_or("mod id missing")?;
    Ok((mod_name.to_string(), mod_id.to_string(), rest.to_string()))
}

fn parse_warning(text: &str) -> Result<Vec<ModVersionConflictInfo>, &'static str> {
    let text = text.strip_prefix("Warnings were found!").ok_or("prefix missing")?.trim_ascii_start();
    let mut vec: Vec<ModVersionConflictInfo> = Vec::new();
    let mut mod_line: Option<(String, String, String)> = None;
    for line in text.lines() {
        // Message on previous line, action on this line
        if let Some(msg) = mod_line.take() {
            let (_, action) = line.split_once("- ").ok_or("action prefix missing")?;
            let (mod_name, normalized_mod_name, message) = msg;
            let conflict = ModVersionConflictInfo {
                mod_name,
                normalized_mod_name,
                message,
                action: action.to_string(),
            };
            vec.push(conflict);
        }
        // Message on this line, action on next line
        else {
            mod_line = Some(parse_mod_line(line)?);
        }
    }
    Ok(vec)
}

pub(crate) fn mod_version_conflicts(entry: &LogEntry) -> Option<Issue> {
    if entry.prefix.level != "WARN" {
        return None;
    }
    if entry.prefix.thread != "main" {
        return None;
    }
    parse_warning(&entry.contents).map(Issue::ModVersionConflicts).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let text = "[21:00:08] [main/WARN]: Warnings were found!
 - Mod 'Create Enchantment Industry' (create_enchantment_industry) 1.2.16 recommends version 0.5.1-f-build.1335 mc1.20.1 of mod 'Create' (create), but only the wrong version is present: 0.5.1-j-build.1631 mc1.20.1!
	 - You should install version 0.5.1-f-build.1335 mc1.20.1 of mod 'Create' (create) for the optimal experience.
 - Mod 'Elytra Trims' (elytratrims) 3.9.3 conflicts with any version of mod 'KubeJS' (kubejs), which is present with the following versions: 2001.6.5-build.16!
	 - While this won't prevent you from starting the game, the developer(s) of mod 'Elytra Trims' (elytratrims) have found that this combination may cause issues. You should remove one of the mods or check for updates that resolve the issue.
 - Mod 'Figura' (figura) 0.1.5 1.20.1 conflicts with any version of mod 'ImmediatelyFast' (immediatelyfast), which is present with the following versions: 1.5.3 1.20.4!
	 - While this won't prevent you from starting the game, the developer(s) of mod 'Figura' (figura) have found that this combination may cause issues. You should remove one of the mods or check for updates that resolve the issue.
 - Mod 'Simply Swords' (simplyswords) 1.56.0-1.20.1 recommends version 1.7.2 or later of bettercombat, which is missing!
	 - You should install version 1.7.2 or later of bettercombat for the optimal experience.
[21:00:08] [main/INFO]: Loading 338 mods:
";
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| mod_version_conflicts(e)).next().expect("Failed to determine issue");
        let Issue::ModVersionConflicts(conflicts) = issue else {
            panic!("Expected ModVersionConflicts issue");
        };
        assert_eq!(conflicts.len(), 4);
    }

    #[test]
    fn test_2() {
        let text = "[13:27:52] [main/WARN]: Warnings were found!
 - Mod 'Skyblocker' (skyblocker) 5.3.0+1.21.5 conflicts with any version of mod 'ImmediatelyFast' (immediatelyfast), which is present with the following versions: 1.9.5+1.21.5!
	 - While this won't prevent you from starting the game, the developer(s) of mod 'Skyblocker' (skyblocker) have found that this combination may cause issues. You should remove one of the mods or check for updates that resolve the issue.
[13:27:52] [main/INFO]: Loading 196 mods:
";
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| mod_version_conflicts(e)).next().expect("Failed to determine issue");
        let Issue::ModVersionConflicts(conflicts) = issue else {
            panic!("Expected ModVersionConflicts issue");
        };
        assert_eq!(conflicts.len(), 1);
    }

    #[test]
    fn test_4() {
        let text = "[16:47:55] [main/WARN]: Warnings were found!
 - Mod 'Expanded Ecosphere' (expanded_ecosphere) 3.2.4 conflicts with any version of mod 'William Wythers' Overhauled Overworld' (wwoo), which is present with the following versions: 2.0.0!
	 - While this won't prevent you from starting the game, the developer(s) of mod 'Expanded Ecosphere' (expanded_ecosphere) have found that this combination may cause issues. You should remove one of the mods or check for updates that resolve the issue.
 - Mod 'Exposure' (exposure) 1.7.16 recommends version 0.5.1-f or later of create, which is missing!
	 - You should install version 0.5.1-f or later of create for the optimal experience.
 - Mod 'Simply Swords' (simplyswords) 1.56.0-1.20.1 recommends version 1.7.2 or later of bettercombat, which is missing!
	 - You should install version 1.7.2 or later of bettercombat for the optimal experience.
 - Mod 'Spawn Animations' (spawnanimations) 1.11.3 mod recommends any version of mr_spawn_animationscompats, which is missing!
	 - You should install any version of mr_spawn_animationscompats for the optimal experience.
[16:47:55] [main/INFO]: Loading 425 mods:
";
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| mod_version_conflicts(e)).next().expect("Failed to determine issue");
        let Issue::ModVersionConflicts(conflicts) = issue else {
            panic!("Expected ModVersionConflicts issue");
        };
        assert_eq!(conflicts.len(), 4);
    }

    #[test]
    fn parse_mod_line_test() {
        let mod_line = " - Mod 'Create Enchantment Industry' (create_enchantment_industry) 1.2.16 recommends version 0.5.1-f-build.1335 mc1.20.1 of mod 'Create' (create), but only the wrong version is present: 0.5.1-j-build.1631 mc1.20.1!";
        let (mod_name, mod_id, message) = parse_mod_line(mod_line).expect("Failed to parse mod line");
        assert_eq!(mod_name, "Create Enchantment Industry");
        assert_eq!(mod_id, "create_enchantment_industry");
        assert_eq!(message, "1.2.16 recommends version 0.5.1-f-build.1335 mc1.20.1 of mod 'Create' (create), but only the wrong version is present: 0.5.1-j-build.1631 mc1.20.1!");
    }
}