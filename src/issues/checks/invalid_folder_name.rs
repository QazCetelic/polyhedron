use crate::issues::issue::Issue;

fn invalid_folder_name(folder: &str) -> Option<Issue> {
    folder.contains('!').then_some(Issue::InvalidFolderName)
}

#[cfg(test)]
mod tests {
    use crate::header::index::IndexedLogHeader;

    use super::*;

    #[test]
    fn matches_invalid_folder_name() {
        let header_fragment = "Prism Launcher version: 9.4 (archlinux)


Launched instance in online mode

login.microsoftonline.com resolves to:
    [**.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, **.**.**.**, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****]


session.minecraft.net resolves to:
    [**.**.**.**, **.**.**.**, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****]


textures.minecraft.net resolves to:
    [**.**.**.**, **.**.**.**, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****]


api.mojang.com resolves to:
    [**.**.**.**, **.**.**.**, ****:****:****:****:****:****:****:****, ****:****:****:****:****:****:****:****]


Minecraft folder is:
/home/********/.local/share/PrismLauncher/instances/Reclamation - Reclaim the World!/minecraft


Java path is:
/home/********/.local/share/PrismLauncher/java/eclipse_temurin_jre17.0.16 8/bin/java

Main Class:
  io.github.zekerzhayard.forgewrapper.installer.Main
";
        let indexed = IndexedLogHeader::index_header(header_fragment);
        let folder = indexed.get_mc_folder_location().expect("Failed to get mc folder");
        let issue = invalid_folder_name(&folder).expect("Failed to determine issue");
        assert_eq!(issue, Issue::InvalidFolderName);
    }
}