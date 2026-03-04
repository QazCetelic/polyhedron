use std::collections::{BTreeMap, BTreeSet};

use crate::{issues::issue::Issue, parse::stacktrace::model::Stacktrace};

pub(crate) fn check_mods_in_stacktrace_namespace<'a>(mod_lookup_map: &BTreeMap<String, String>, stacktraces: &[Stacktrace]) -> Option<Issue> {
	let mut mods = BTreeSet::new();
	for stacktrace in stacktraces {
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
    use crate::{entries::entry::LogEntry, header::index::IndexedLogHeader, parse::{crash_report::CrashReport, stacktrace::model::Stacktrace}};

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
        let issue = check_mods_in_stacktrace_namespace(&mod_lookup_map, &crash_report.stacktrace).expect("Failed to find issue");
		let Issue::ModsFoundInStacktraceNamespace(mods) = issue else { panic!("Not the right issue"); };
		assert_eq!(mods.len(), 1);
		assert!(mods.contains("waveycapes-fabric-1.8.2-mc1.21.11"))
    }

	#[test]
    fn detect_mods_2() {
        let header_fragment = r#"
Mods:
  [✔] 3dSkinLayers-forge-mc1.8.9-1.2.0
  [✔] Aoneconfigbootstrap-1.8.9-forge-1.0.3
  [✔] Autocorrect-1.1 (1.8.9)
  [✔] AutoGG_Reimagined-1.8.9-forge-1.1
  [✔] AutoParty-1.0
  [✔] BedWar-0.1.7.Pre.1
  [✔] Bedwars_Mod-1.8.9-forge-0.2.5.3
  [✔] BetterFps-1.2.0
  [✔] BetterHurtCam-2.2.0
  [✔] Chatting-1.8.9-forge-1.5.3
  [✔] ColorSaturation-1.8.9-forge-1.0.0
  [✔] Controlling-7.0.0.1
  [✔] CrashPatch-1.8.9-forge-2.0.2
  [✔] entityculling-forge-mc1.8.9-1.5.0
  [✔] essential_1-3-1-1_forge_1-8-9
  [✔] EvergreenHUD-1.8.9-forge-2.1.4
  [✔] foamfix-0.6.3a-anarchy-1.8.x
  [✔] HitDelayFix1.8.9
  [✔] Hytools-1.8
  [✔] Ksyxis-1.3.4
  [✔] LevelHead-8.2.2 (1.8.9)
  [✔] microoptimizations-1.0.0
  [✔] ModernKeyBinding-Forge-1.8.9-2.1.0
  [✔] MouseTweaks-2.6.2-mc1.8.9
  [✔] NotSoEssential-Forge-1.0.3
  [✔] Optibye-1.0.0-dep
  [✔] OptiFine_1.8.9_HD_U_M5
  [✔] OverflowAnimations-1.8.9-forge-2.2.5
  [✔] Patcher-1.8.9 (1.8.9)
  [✔] PerspectiveModv4-master-4.5
  [✔] ping-1.8.9-2.0.4
  [✔] PolyCrosshair-1.8.9-forge-1.0.3
  [✔] PolyNametag-1.8.9-forge-1.0.9
  [✔] PolySprint-1.8.9-forge-1.0.1
  [✔] QuickJoin-1.8.9-forge-2.9
  [✔] RawInput-0.1.8 1.8.9-forge
  [✔] RewardClaim-1.0.7
  [✔] simpletimechanger-1.0.2
  [✔] TNT Time-1.1 (1.8.9)
  [✔] VanillaHUD-1.8.9-forge-2.1.1
  [✔] veloxcaelo-1.1.0
  [✔] weaponmaster_ydm-forge-1.8.9-4.2.3
  [✔] zergatul.freecam-0.1.1-forge-1.8.9

Params:
  --username  --version 1.8.9 --gameDir C:/Users/********/AppData/Roaming/PrismLauncher/instances/Enhanced Bedwars/minecraft --assetsDir C:/Users/********/AppData/Roaming/PrismLauncher/assets --assetIndex 1.8 --uuid  --accessToken  --userProperties  --userType  --tweakClass net.minecraftforge.fml.common.launcher.FMLTweaker
  "#;
        let indexed_header = IndexedLogHeader::index_header(header_fragment);

        let log_fragment = r#"
[17:07:10] [Sound Library Loader/INFO]: Sound engine started
[17:07:11] [Thread-18/ERROR] [FML]: Splash thread Exception
java.lang.NullPointerException: Parameter specified as non-null is null: method gg.essential.gui.common.ExtensionsKt.or, parameter other
	at gg.essential.gui.common.ExtensionsKt.or(extensions.kt) ~[ExtensionsKt.class:?]
	at gg.essential.gui.common.MenuButton.<init>(MenuButton.kt:93) ~[MenuButton.class:?]
	at gg.essential.gui.common.MenuButton.<init>(MenuButton.kt:159) ~[MenuButton.class:?]
	at gg.essential.gui.common.MenuButton.<init>(MenuButton.kt:147) ~[MenuButton.class:?]
	at gg.essential.gui.menu.RightSideBarNew$inviteOrHostButton$1$3$1.invoke(RightSideBarNew.kt:213) ~[RightSideBarNew$inviteOrHostButton$1$3$1.class:?]
	at gg.essential.gui.menu.RightSideBarNew$inviteOrHostButton$1$3$1.invoke(RightSideBarNew.kt:212) ~[RightSideBarNew$inviteOrHostButton$1$3$1.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$if_$2.invoke(layout.kt:67) ~[LayoutScope$if_$2.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$if_$2.invoke(layout.kt:67) ~[LayoutScope$if_$2.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach$add(layout.kt:138) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach$update(layout.kt:163) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.access$forEach$update(layout.kt:31) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$forEach$1.invoke(layout.kt:179) ~[LayoutScope$forEach$1.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$forEach$1.invoke(layout.kt:176) ~[LayoutScope$forEach$1.class:?]
	at gg.essential.gui.elementa.state.v2.impl.basic.Node.update(impl.kt:255) ~[Node.class:?]
	at gg.essential.gui.elementa.state.v2.impl.basic.MarkThenPushAndPullImpl.effect(impl.kt:62) ~[MarkThenPushAndPullImpl.class:?]
	at gg.essential.gui.elementa.state.v2.StateKt.effect(state.kt:123) ~[StateKt.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach(layout.kt:176) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.if_(layout.kt:67) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.if_$default(layout.kt:66) ~[LayoutScope.class:?]
	at gg.essential.gui.menu.RightSideBarNew$inviteOrHostButton$1$3.invoke(RightSideBarNew.kt:212) ~[RightSideBarNew$inviteOrHostButton$1$3.class:?]
	at gg.essential.gui.menu.RightSideBarNew$inviteOrHostButton$1$3.invoke(RightSideBarNew.kt:203) ~[RightSideBarNew$inviteOrHostButton$1$3.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$if_$2.invoke(layout.kt:67) ~[LayoutScope$if_$2.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$if_$2.invoke(layout.kt:67) ~[LayoutScope$if_$2.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach$add(layout.kt:138) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach$update(layout.kt:163) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.access$forEach$update(layout.kt:31) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$forEach$1.invoke(layout.kt:179) ~[LayoutScope$forEach$1.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$forEach$1.invoke(layout.kt:176) ~[LayoutScope$forEach$1.class:?]
	at gg.essential.gui.elementa.state.v2.impl.basic.Node.update(impl.kt:255) ~[Node.class:?]
	at gg.essential.gui.elementa.state.v2.impl.basic.MarkThenPushAndPullImpl.effect(impl.kt:62) ~[MarkThenPushAndPullImpl.class:?]
	at gg.essential.gui.elementa.state.v2.StateKt.effect(state.kt:123) ~[StateKt.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach(layout.kt:176) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.if_(layout.kt:67) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.else(layout.kt:83) ~[LayoutScope.class:?]
	at gg.essential.gui.menu.RightSideBarNew$inviteOrHostButton$1.invoke(RightSideBarNew.kt:203) ~[RightSideBarNew$inviteOrHostButton$1.class:?]
	at gg.essential.gui.menu.RightSideBarNew$inviteOrHostButton$1.invoke(RightSideBarNew.kt:202) ~[RightSideBarNew$inviteOrHostButton$1.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler$Companion$mountWithProxy$2$1.invoke(ScreenWithProxiesHandler.kt:120) ~[ScreenWithProxiesHandler$Companion$mountWithProxy$2$1.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler$Companion$mountWithProxy$2$1.invoke(ScreenWithProxiesHandler.kt:119) ~[ScreenWithProxiesHandler$Companion$mountWithProxy$2$1.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.invoke(layout.kt:51) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.ContainersKt.box(containers.kt:45) ~[ContainersKt.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler$Companion$mountWithProxy$2.invoke(ScreenWithProxiesHandler.kt:119) ~[ScreenWithProxiesHandler$Companion$mountWithProxy$2.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler$Companion$mountWithProxy$2.invoke(ScreenWithProxiesHandler.kt:117) ~[ScreenWithProxiesHandler$Companion$mountWithProxy$2.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$if_$2.invoke(layout.kt:67) ~[LayoutScope$if_$2.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$if_$2.invoke(layout.kt:67) ~[LayoutScope$if_$2.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach$add(layout.kt:138) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach$update(layout.kt:163) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.access$forEach$update(layout.kt:31) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$forEach$1.invoke(layout.kt:179) ~[LayoutScope$forEach$1.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope$forEach$1.invoke(layout.kt:176) ~[LayoutScope$forEach$1.class:?]
	at gg.essential.gui.elementa.state.v2.impl.basic.Node.update(impl.kt:255) ~[Node.class:?]
	at gg.essential.gui.elementa.state.v2.impl.basic.MarkThenPushAndPullImpl.effect(impl.kt:62) ~[MarkThenPushAndPullImpl.class:?]
	at gg.essential.gui.elementa.state.v2.StateKt.effect(state.kt:123) ~[StateKt.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.forEach(layout.kt:176) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.if_(layout.kt:67) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.if_$default(layout.kt:66) ~[LayoutScope.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler$Companion.mountWithProxy(ScreenWithProxiesHandler.kt:117) ~[ScreenWithProxiesHandler$Companion.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler$Companion.mountWithProxy$default(ScreenWithProxiesHandler.kt:114) ~[ScreenWithProxiesHandler$Companion.class:?]
	at gg.essential.gui.menu.RightSideBarNew.inviteOrHostButton(RightSideBarNew.kt:202) ~[RightSideBarNew.class:?]
	at gg.essential.gui.menu.RightSideBarNew.access$inviteOrHostButton(RightSideBarNew.kt:67) ~[RightSideBarNew.class:?]
	at gg.essential.gui.menu.RightSideBarNew$1$1.invoke(RightSideBarNew.kt:124) ~[RightSideBarNew$1$1.class:?]
	at gg.essential.gui.menu.RightSideBarNew$1$1.invoke(RightSideBarNew.kt:120) ~[RightSideBarNew$1$1.class:?]
	at gg.essential.gui.layoutdsl.LayoutScope.invoke(layout.kt:51) ~[LayoutScope.class:?]
	at gg.essential.gui.layoutdsl.ContainersKt.row(containers.kt:67) ~[ContainersKt.class:?]
	at gg.essential.gui.layoutdsl.ContainersKt.row(containers.kt:52) ~[ContainersKt.class:?]
	at gg.essential.gui.layoutdsl.ContainersKt.row$default(containers.kt:48) ~[ContainersKt.class:?]
	at gg.essential.gui.menu.RightSideBarNew$1.invoke(RightSideBarNew.kt:120) ~[RightSideBarNew$1.class:?]
	at gg.essential.gui.menu.RightSideBarNew$1.invoke(RightSideBarNew.kt:111) ~[RightSideBarNew$1.class:?]
	at gg.essential.gui.layoutdsl.LayoutKt.layoutAsColumn(layout.kt:392) ~[LayoutKt.class:?]
	at gg.essential.gui.menu.RightSideBarNew.<init>(RightSideBarNew.kt:111) ~[RightSideBarNew.class:?]
	at gg.essential.handlers.PauseMenuDisplay.initContent(PauseMenuDisplay.kt:214) ~[PauseMenuDisplay.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler$Companion$forMainMenu$1.invoke(ScreenWithProxiesHandler.kt:99) ~[ScreenWithProxiesHandler$Companion$forMainMenu$1.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler$Companion$forMainMenu$1.invoke(ScreenWithProxiesHandler.kt:98) ~[ScreenWithProxiesHandler$Companion$forMainMenu$1.class:?]
	at gg.essential.gui.proxies.ScreenWithProxiesHandler.initGui(ScreenWithProxiesHandler.kt:60) ~[ScreenWithProxiesHandler.class:?]
	at net.minecraft.client.gui.GuiMainMenu.handler$bgi001$addProxyButtons(GuiMainMenu.java:1799) ~[aya.class:?]
	at net.minecraft.client.gui.GuiMainMenu.func_73866_w_(GuiMainMenu.java:263) ~[aya.class:?]
	at net.minecraft.client.gui.GuiScreen.func_146280_a(GuiScreen.java:502) ~[axu.class:?]
	at net.minecraft.client.gui.GuiScreen.func_175273_b(GuiScreen.java:697) ~[axu.class:?]
	at net.minecraft.client.Minecraft.func_71370_a(Minecraft.java:1595) ~[ave.class:?]
	at net.minecraftforge.fml.client.SplashProgress$3.clearGL(SplashProgress.java:463) ~[SplashProgress$3.class:?]
	at net.minecraftforge.fml.client.SplashProgress$3.run(SplashProgress.java:384) ~[SplashProgress$3.class:?]
	at java.lang.Thread.run(Thread.java:745) [?:1.8.0_51]
"#;
		let entries = LogEntry::from_lines(log_fragment.lines());
		let stacktraces = Stacktrace::from_lines(entries.last().unwrap().contents.lines()).collect::<Vec<Stacktrace>>();
		assert!(!stacktraces.is_empty());

		let mod_lookup_map = indexed_header.get_mod_name_lookup_map().unwrap();
        let issue = check_mods_in_stacktrace_namespace(&mod_lookup_map, &stacktraces).expect("Failed to find issue");
		let Issue::ModsFoundInStacktraceNamespace(mods) = issue else { panic!("Not the right issue"); };
		assert_eq!(mods.len(), 1);
		assert!(mods.contains("essential_1-3-1-1_forge_1-8-9"))
    }
}