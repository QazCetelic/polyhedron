use crate::{header::{extract::ModInfo, index::IndexedLogHeader, mc_version::McVersion}, issues::issue::Issue};

fn optifine(version: Option<McVersion>, mods: &[ModInfo]) -> Option<Issue> {
    if let Some(ver) = version {
        // Before 1.16? Ignore OptiFine usage
        if ver.major == 1 && ver.minor < 16 {
            return None;
        }
    }

    mods.iter().any(|m| m.enabled && (m.name.starts_with("OptiFine") || m.name.starts_with("optifabric"))).then_some(Issue::Optifine)
}

pub(crate) fn optifine_header(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    let mods = header.get_mods()?;
    optifine(header.get_mc_version(), &mods)
}

#[cfg(test)]
mod tests {
    use crate::header::index::IndexedLogHeader;

    use super::*;

    #[test]
    fn ignore_old_mc() {
        let header_fragment = "

Mods:
  [✔] ctjs-2.2.1-1.8.9
  [✔] Odin-1.3.1
  [✔] OptiFine_1.8.9_HD_U_M5
  [✔] patcher-1.8.9.temp
  [✔] PolyBlur-1.8.9-forge-1.0.2
  [✔] Skytils-1.10.9

Params:
  --username  --version 1.8.9 --gameDir C:/Users/********/AppData/Roaming/PrismLauncher/instances/Enhanced Bedwars/minecraft --assetsDir C:/Users/********/AppData/Roaming/PrismLauncher/assets --assetIndex 1.8 --uuid  --accessToken  --userProperties  --userType  --tweakClass net.minecraftforge.fml.common.launcher.FMLTweaker
  ";
        let indexed = IndexedLogHeader::index_header(header_fragment);
        let mods = indexed.get_mods().expect("Failed to extract mods");

        assert_eq!(optifine(indexed.get_mc_version(), &mods), None);
    }

    #[test]
    fn matches_optifine() {
        let header_fragment = "

Mods:
  [✔] ctjs-2.2.1-1.8.9
  [✔] Odin-1.3.1
  [✔] OptiFine_1.8.9_HD_U_M5
  [✔] patcher-1.8.9.temp
  [✔] PolyBlur-1.8.9-forge-1.0.2
  [✔] Skytils-1.10.9

Params:
  --username  --version 1.21.1 --gameDir D:/PrismLauncher-Windows-MinGW-w64-Portable-8.4/instances/RPG moddat NeoFabric 1.21.1/minecraft --assetsDir D:/PrismLauncher-Windows-MinGW-w64-Portable-8.4/assets --assetIndex 17 --uuid  --accessToken  --userType  --versionType release --fml.neoForgeVersion 21.1.215 --fml.fmlVersion 4.0.42 --fml.mcVersion 1.21.1 --fml.neoFormVersion 20240808.144430 --launchTarget forgeclient
  ";
        let indexed = IndexedLogHeader::index_header(header_fragment);
        let mods = indexed.get_mods().expect("Failed to extract mods");

        let issue = optifine(indexed.get_mc_version(), &mods).expect("Failed to determine issue");
        assert_eq!(issue, Issue::Optifine);
    }
}