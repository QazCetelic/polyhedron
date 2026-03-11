use crate::{entries::entry::LogEntry, issues::issue::Issue};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IncompatibleModsInfo {
    pub solution: String,
    pub details: String,
}

pub(crate) fn incompatible_mods(entry: &LogEntry) -> Option<Issue> {
    let mut lines = entry.contents.lines().peekable();
    if lines.next()? != "Incompatible mods found!" { return None; }
    if lines.next()? != "net.fabricmc.loader.impl.FormattedException: Some of your mods are incompatible with the game or each other!" { return None; }
    if lines.next()? != "A potential solution has been determined, this may resolve your problem:" { return None; }
    let mut solution = String::new();
    while *lines.peek()? != "More details:" && !lines.peek()?.starts_with("\tat") {
        solution.push_str(lines.next()?);
        solution.push('\n');
    }
    if lines.next()? != "More details:" { return None; }
    let mut details = String::new();
    while !lines.peek()?.starts_with("\tat") {
        details.push_str(lines.next()?);
        details.push('\n');
    }
    
    Some(Issue::IncompatibleMods(Box::new(IncompatibleModsInfo {
        solution,
        details,
    })))
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn example_1() {
        let log = r#"[22:00:56] [main/ERROR]: Incompatible mods found!
net.fabricmc.loader.impl.FormattedException: Some of your mods are incompatible with the game or each other!
A potential solution has been determined, this may resolve your problem:
	 - Replace mod 'Nvidium' (nvidium) 0.3.1 with any version that is compatible with:
		 - minecraft, version 1.21
		 - sodium 0.6.13 mc1.21.1
	 - Replace mod 'Fabric API' (fabric-api) 0.102.0 1.21 with version 0.104.0 1.21.1 or later.
	 - Replace 'Minecraft' (minecraft) 1.21.1 with version 1.21.
More details:
	 - Mod 'Fabric API' (fabric-api) 0.102.0 1.21 requires any version between 1.21- (inclusive) and 1.21.1- (exclusive) of 'Minecraft' (minecraft), but only the wrong version is present: 1.21.1!
	 - Mod 'Lithium' (lithium) 0.13.1 requires version 1.21 of 'Minecraft' (minecraft), but only the wrong version is present: 1.21.1!
	 - Mod 'Nvidium' (nvidium) 0.3.1 requires version 0.5.9 or version 0.5.11 of mod 'Sodium' (sodium), but only the wrong version is present: 0.6.13 mc1.21.1!
	 - Mod 'YetAnotherConfigLib' (yet_another_config_lib_v3) 3.8.0 1.21.1-fabric requires version 0.104.0 1.21.1 or later of mod 'Fabric API' (fabric-api), but only the wrong version is present: 0.102.0 1.21!
	at net.fabricmc.loader.impl.FormattedException.ofLocalized(FormattedException.java:51) ~[fabric-loader-0.17.3.jar:?]"#;
        let entries = LogEntry::from_lines(log.lines());
        let entry = entries.first().expect("Failed to get entry");
        let Issue::IncompatibleMods(info) = incompatible_mods(&entry).expect("Failed to determine issue") else { panic!("Wrong issue") };
        assert!(info.solution.contains("Nvidium"));
        assert!(info.details.contains("Fabric API"));
    }
}