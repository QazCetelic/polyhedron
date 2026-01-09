use crate::{header::{extract::ModInfo, index::IndexedLogHeader}, issues::issue::Issue};

fn optifine(mods: &[ModInfo]) -> Option<Issue> {
    mods.iter().any(|m| m.enabled && (m.name.starts_with("OptiFine") || m.name.starts_with("optifabric"))).then_some(Issue::Optifine)
}

pub(crate) fn optifine_header(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    let mods = header.get_mods()?;
    optifine(&mods)
}

#[cfg(test)]
mod tests {
    use crate::header::index::IndexedLogHeader;

    use super::*;

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

Params:";
        let indexed = IndexedLogHeader::index_header(header_fragment);
        let mods = indexed.get_mods().expect("Failed to extract mods");

        let issue = optifine(&mods).expect("Failed to determine issue");
        assert_eq!(issue, Issue::Optifine);
    }
}