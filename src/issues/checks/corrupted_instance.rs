use lazy_regex::regex;

use crate::issues::issue::Issue;

pub(crate) fn corrupted_instance(header_text: &str) -> Option<Issue> {
    let pack_json_illegal_regex = regex!(r"mmc-pack.json.*illegal value");

    pack_json_illegal_regex.is_match(header_text).then_some(Issue::InstanceDataCorrupted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_corrupted_instance() {
        let header_fragment = r#"Prism Launcher version: 9.4 (official)


Launched instance in online mode

login.microsoftonline.com resolves to:
    [**.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**]


session.minecraft.net resolves to:
    [**.**.**.**, **.**.**.**]


textures.minecraft.net resolves to:
    [**.**.**.**, **.**.**.**]


api.mojang.com resolves to:
    [**.**.**.**, **.**.**.**]


Minecraft folder is:
C:/Users/********/AppData/Roaming/PrismLauncher/instances/1.18.2(1)/minecraft


Instance update failed because: Couldn't parse C:/Users/********/AppData/Roaming/PrismLauncher/instances/1.18.2(1)/mmc-pack.json as json: illegal value


Log upload triggered at: 05 Oct 2025 20:51:11 -0700"#;
        let issue = corrupted_instance(&header_fragment).expect("Failed to determine issue");
        assert_eq!(issue, Issue::InstanceDataCorrupted);
    }
}