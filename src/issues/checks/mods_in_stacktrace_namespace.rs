use std::collections::{BTreeMap, BTreeSet};

use crate::{issues::issue::Issue, parse::crash_report::CrashReport};

pub(crate) fn check_mods_in_stacktrace_namespace<'a>(mod_lookup_map: &BTreeMap<String, String>, report: &CrashReport) -> Option<Issue> {
	let mut mods = BTreeSet::new();
	for stacktrace in &report.stacktrace {
		for line in &stacktrace.lines {
			// Looks for mod name in stacktrace classname
			let parts = line.class.split('.');
			for part in parts {
				if let Some(mod_name) = mod_lookup_map.get(part) {
					mods.insert(mod_name.to_string());
				}
			}
		}
	}
    (!mods.is_empty()).then_some(Issue::ModsFoundInStacktraceNamespace(mods))
}

#[cfg(test)]
mod tests {
    use crate::header::index::IndexedLogHeader;

    use super::*;

    #[test]
    fn detect_mods() {
        let header_fragment = r#"
Mods:
  [✔] skinlayers3d-fabric-1.10.1-mc1.21.11
  [✔] skinshuffle-2.10.1 1.21.11-fabric
  [✔] sodium-fabric-0.8.2 mc1.21.11
  [✔] sound-physics-remastered-fabric-1.21.11-1.5.1
  [✔] Void-Fog-2.13.2 1.21.11
  [✔] waveycapes-fabric-1.8.2-mc1.21.11
  [✔] xaerominimap-fabric-1.21.11-25.3.5
  [✔] yet_another_config_lib_v3-3.8.2 1.21.11-fabric
  [✔] zoomify-2.15.1 1.21.11

Params:
  --username  --version 1.21.11 --gameDir C:/Users/********/AppData/Roaming/PrismLauncher/instances/All 3 Required/minecraft --assetsDir C:/Users/********/AppData/Roaming/PrismLauncher/assets --assetIndex 29 --uuid  --accessToken  --versionType release
"#;
        let indexed_header = IndexedLogHeader::index_header(header_fragment);

        let crash_report_fragment = r#"
---- Minecraft Crash Report ----
// Hey, that tickles! Hehehe!
Time: 2026-01-19 10:10:40
Description: Unexpected error
java.lang.NullPointerException: Cannot invoke "net.minecraft.class_11901.method_52814()" because "avatar" is null
	at knot//dev.tr7zw.transition.mc.PlayerUtil.getPlayerCape(PlayerUtil.java:119)
	at knot//dev.tr7zw.transition.mc.entitywrapper.PlayerWrapper.getCapeTexture(PlayerWrapper.java:72)
	at knot//dev.tr7zw.waveycapes.CustomCapeRenderer.getCapeRenderer(CustomCapeRenderer.java:433)
	at knot//dev.tr7zw.waveycapes.CustomCapeRenderer.render(CustomCapeRenderer.java:48)
	at knot//net.minecraft.class_11684.handler$ebg000$waveycapes$renderCapes(class_11684.java:2035)
	at knot//net.minecraft.class_11684.method_73002(class_11684.java:57)
	at knot//net.minecraft.class_11235.method_70909(class_11235.java:42)
	at knot//net.minecraft.class_11235.method_70905(class_11235.java:15)
	at knot//net.minecraft.class_11239.method_70913(class_11239.java:57)
	at knot//net.minecraft.class_11228.handler$dmb000$skinshuffle$skyblocker$instancedGuiElementRendering(class_11228.java:1915)
	at knot//net.minecraft.class_11228.method_70888(class_11228.java)
	at knot//net.minecraft.class_11228.method_71056(class_11228.java:405)
	at knot//net.minecraft.class_11246.method_71065(class_11246.java:209)
	at knot//net.minecraft.class_11246.method_71061(class_11246.java:244)
	at knot//net.minecraft.class_11246.method_71061(class_11246.java:246)
	at knot//net.minecraft.class_11246.method_71074(class_11246.java:239)
	at knot//net.minecraft.class_11246.method_71071(class_11246.java:205)
	at knot//net.minecraft.class_11228.method_70893(class_11228.java:404)
	at knot//net.minecraft.class_11228.method_71290(class_11228.java:203)
	at knot//net.minecraft.class_11228.method_70890(class_11228.java:168)
	at knot//net.minecraft.class_757.method_3192(class_757.java:585)
	at knot//net.minecraft.class_310.method_1523(class_310.java:1393)
	at knot//net.minecraft.class_310.method_1514(class_310.java:966)
	at knot//net.minecraft.client.main.Main.main(Main.java:250)
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:514)
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72)
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:115)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
A detailed walkthrough of the error, its code path and all known details is as follows:
---------------------------------------------------------------------------------------
-- Head --
Thread: Render thread
"#;
        let crash_report = CrashReport::parse(crash_report_fragment).expect("Failed to parse crash report");

		let mod_lookup_map = indexed_header.get_mod_name_lookup_map().unwrap();
        let issue = check_mods_in_stacktrace_namespace(&mod_lookup_map, &crash_report).expect("Failed to find issue");
		let Issue::ModsFoundInStacktraceNamespace(mods) = issue else { panic!("Not the right issue"); };
		assert_eq!(mods.len(), 1);
		assert!(mods.contains("waveycapes-fabric-1.8.2-mc1.21.11"))
    }
}