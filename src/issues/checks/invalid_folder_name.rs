use crate::{header::index::IndexedLogHeader, issues::issue::Issue};

fn invalid_folder_name(folder: &str) -> Option<Issue> {
    folder.contains('!').then_some(Issue::InvalidFolderName('!'))
}

pub(crate) fn invalid_folder_name_header(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    let folder_name = header.get_mc_folder_location()?;
    invalid_folder_name(folder_name)
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
        assert_eq!(issue, Issue::InvalidFolderName('!'));
    }

    #[test]
    fn valid_names() {
        let names = [
            "/home/********/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/destoy/minecraft",
            "C:/Users/********/AppData/Roaming/PrismLauncher/instances/1.21.1(1)/minecraft",
            "C:/Users/********/AppData/Roaming/PrismLauncher/instances/Vulkan Optimized/minecraft"
        ];
        for name in names {
            assert_eq!(invalid_folder_name(name), None);
        }
    }
}