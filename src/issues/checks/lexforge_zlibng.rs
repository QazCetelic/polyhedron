use crate::{header::index::IndexedLogHeader, issues::issue::Issue};

// Input should be header beyond building_processors index
pub(crate) fn lexforge_zlibng(header_text: &str) -> Option<Issue> {
    if header_text.contains("Processor failed, invalid outputs:") {
        Some(Issue::LexforgeZlibng)
    }
    else {
        None
    }
}

pub(crate) fn lexforge_zlibng_header(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    lexforge_zlibng(header.text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_lexforge_zlibng() {
        let text = "  Data  assets/minecraft/models/item/nether_brick_fence.json
  Data  assets/minecraft/models/item/calcite.json
  Data  assets/minecraft/models/item/coal_ore.json
  Data  assets/minecraft/models/item/crimson_door.json
  Data  assets/minecraft/models/item/bamboo_mosaic_slab.json
  Data  assets/minecraft/models/item/turtle_helmet_quartz_trim.json
  Processor failed, invalid outputs:
    /home/********/.local/share/PrismLauncher/libraries/net/minecraft/client/1.20.1-20230612.114412/client-1.20.1-20230612.114412-slim.jar
      Expected: de86b035d2da0f78940796bb95c39a932ed84834
      Actual:   65de7c2f734b5288bfaff943eb9849b979332c47
    /home/********/.local/share/PrismLauncher/libraries/net/minecraft/client/1.20.1-20230612.114412/client-1.20.1-20230612.114412-extra.jar
      Expected: 8c5a95cbce940cfdb304376ae9fea47968d02587
      Actual:   093305ab07def4677e31e58648b25eeeb5815875
Process exited with code 0.
Clipboard copy at: 16 Dec 2025 17:22:10 +0200";

        let issue = lexforge_zlibng(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::LexforgeZlibng);
    }
}