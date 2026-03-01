use std::collections::{BTreeMap, BTreeSet};

use crate::{header::extract::ModInfo, issues::issue::Issue, parse::crash_report::CrashReport};

pub(crate) fn check_mods_in_stacktrace_info<'a>(mods_in_header: &[ModInfo], report: &CrashReport) -> Option<Issue> {
    let mods_set = mods_in_header.iter().filter(|m| m.enabled).map(|m| m.name.to_string()).collect::<BTreeSet<String>>();
	let mut mods = BTreeSet::new();
	for stacktrace in &report.stacktrace {
		for line in &stacktrace.lines {
			if let Some(info) = line.extract_source_info() {
                if let Some(jar_name) = info.source_name.strip_suffix(".jar") { 
                    if mods_set.contains(jar_name) {
                        mods.insert(info.source_name);
                    }
                }
            }
		}
	}
    (!mods.is_empty()).then_some(Issue::ModsFoundInStacktraceInfo(mods))
}

#[cfg(test)]
mod tests {
    use crate::header::index::IndexedLogHeader;

    use super::*;

    #[test]
    fn detect_mods() {
        let header_fragment = r#"
Mods:
  [✔] continuity-3.0.1-beta.2 1.21.10
  [✔] coolrain-1.3.1-1.21.10
  [✔] DiagonalFences-v21.10.0 mc1.21.10-Fabric
  [✘] dynamicanimations-1.0.0.jar (disabled)
  [✔] dynamiccrosshair-9.10 1.21.9-forge
  [✔] fabric-api-0.138.4 1.21.10
  [✔] fabric-loader-0.16.14-1.21.5
  [✔] ferritecore-8.0.2-neoforge
  [✔] ForgeConfigAPIPort-v21.10.1 mc1.21.10-Forge
  [✔] highlight-fabric-1.21.9-3.9.0
  [✔] ImmediatelyFast-Fabric-1.13.4 1.21.10
  [✔] iris-fabric-1.9.7 mc1.21.10
  [✔] lithium-fabric-0.20.1 mc1.21.10
  [✔] NoChatReports-FORGE-1.21.10-v2.16.0
  [✘] omnilook-0.1.jar (disabled)
  [✔] particlerain-4.0.0-beta.5 1.21.9-fabric
  [✔] ShoulderSurfing-Forge-1.21.10-4.17.1
  [✔] sodium-extra-fabric-0.7.1 mc1.21.10
  [✔] sodium-fabric-0.7.3 mc1.21.10
  [✔] sound-physics-remastered-fabric-1.21.10-1.5.1
  [✔] SubtleEffects-fabric-1.21.10-1.13.2-hotfix.1
  [✘] vanilla-refresh-1.4.28b.jar (disabled)
  [✘] vanilla-refresh-1.4.28b.jar.duplicate (disabled)
  [✔] vein_miner-1.0
  [✔] Xaeros_Minimap_25.2.15_Fabric_1.21.9
  [✔] XaerosWorldMap_1.39.17_Fabric_1.21.9
  [✔] XaerosWorldMap_1.39.17_Fabric_1.21.9.jar
  [✔] Zoomify-2.14.6 1.21.9

"#;
        let indexed_header = IndexedLogHeader::index_header(header_fragment);

        let crash_report_fragment = r#"
---- Minecraft Crash Report ----
// My bad.

Time: 2025-12-30 20:58:41
Description: Unexpected error

java.util.ServiceConfigurationError: com.github.exopandora.shouldersurfing.api.client.IShoulderSurfing: Provider com.github.exopandora.shouldersurfing.client.ShoulderSurfingImpl could not be instantiated
	at java.base/java.util.ServiceLoader.fail(ServiceLoader.java:586) ~[?:?]
	at java.base/java.util.ServiceLoader$ProviderImpl.newInstance(ServiceLoader.java:813) ~[?:?]
	at java.base/java.util.ServiceLoader$ProviderImpl.get(ServiceLoader.java:729) ~[?:?]
	at java.base/java.util.ServiceLoader$3.next(ServiceLoader.java:1403) ~[?:?]
	at java.base/java.util.ServiceLoader.findFirst(ServiceLoader.java:1813) ~[?:?]
	at TRANSFORMER/shouldersurfing@1.21.10-4.17.1/com.github.exopandora.shouldersurfing.api.client.ShoulderSurfing.<clinit>(ShoulderSurfing.java:7) ~[ShoulderSurfing-Forge-1.21.10-4.17.1.jar!/:1.21.10-4.17.1]
	at TRANSFORMER/shouldersurfing@1.21.10-4.17.1/com.github.exopandora.shouldersurfing.client.ShoulderSurfingImpl.getInstance(ShoulderSurfingImpl.java:441) ~[ShoulderSurfing-Forge-1.21.10-4.17.1.jar!/:1.21.10-4.17.1]
	at TRANSFORMER/minecraft@1.21.10/net.minecraft.client.renderer.GameRenderer.handler$shouldersurf$zcj000$render(GameRenderer.java:1505) ~[forge-1.21.10-60.1.5-client.jar:?]
	at TRANSFORMER/minecraft@1.21.10/net.minecraft.client.renderer.GameRenderer.render(GameRenderer.java) ~[forge-1.21.10-60.1.5-client.jar:?]
	at TRANSFORMER/minecraft@1.21.10/net.minecraft.client.Minecraft.runTick(Minecraft.java:1302) ~[forge-1.21.10-60.1.5-client.jar:?]
	at TRANSFORMER/minecraft@1.21.10/net.minecraft.client.Minecraft.run(Minecraft.java:901) ~[forge-1.21.10-60.1.5-client.jar:?]
	at TRANSFORMER/minecraft@1.21.10/net.minecraft.client.main.Main.main(Main.java:223) ~[minecraft-1.21.10-client.jar:?]
	at java.base/jdk.internal.reflect.DirectMethodHandleAccessor.invoke(DirectMethodHandleAccessor.java:103) ~[?:?]
	at java.base/java.lang.reflect.Method.invoke(Method.java:580) ~[?:?]
	at SECURE-BOOTSTRAP/net.minecraftforge.fmlloader@1.21.10-60.1.5/net.minecraftforge.fml.loading.targets.CommonLaunchHandler.runTarget(CommonLaunchHandler.java:96) ~[fmlloader-1.21.10-60.1.5.jar!/:?]
	at SECURE-BOOTSTRAP/net.minecraftforge.fmlloader@1.21.10-60.1.5/net.minecraftforge.fml.loading.targets.CommonLaunchHandler.lambda$makeService$0(CommonLaunchHandler.java:79) ~[fmlloader-1.21.10-60.1.5.jar!/:?]
	at SECURE-BOOTSTRAP/cpw.mods.modlauncher@10.2.4/cpw.mods.modlauncher.LaunchServiceHandler.launch(LaunchServiceHandler.java:77) [modlauncher-10.2.4.jar!/:?]
	at SECURE-BOOTSTRAP/cpw.mods.modlauncher@10.2.4/cpw.mods.modlauncher.LaunchServiceHandler.launch(LaunchServiceHandler.java:97) [modlauncher-10.2.4.jar!/:?]
	at SECURE-BOOTSTRAP/cpw.mods.modlauncher@10.2.4/cpw.mods.modlauncher.Launcher.run(Launcher.java:116) [modlauncher-10.2.4.jar!/:?]
	at SECURE-BOOTSTRAP/cpw.mods.modlauncher@10.2.4/cpw.mods.modlauncher.Launcher.main(Launcher.java:75) [modlauncher-10.2.4.jar!/:?]
	at SECURE-BOOTSTRAP/cpw.mods.modlauncher@10.2.4/cpw.mods.modlauncher.BootstrapEntry.main(BootstrapEntry.java:17) [modlauncher-10.2.4.jar!/:?]
	at net.minecraftforge.bootstrap@2.1.7/net.minecraftforge.bootstrap.Bootstrap.moduleMain(Bootstrap.java:188) [bootstrap-2.1.8.jar!/:?]
	at java.base/jdk.internal.reflect.DirectMethodHandleAccessor.invoke(DirectMethodHandleAccessor.java:103) ~[?:?]
	at java.base/java.lang.reflect.Method.invoke(Method.java:580) ~[?:?]
	at net.minecraftforge.bootstrap.Bootstrap.bootstrapMain(Bootstrap.java:133) [bootstrap-2.1.8.jar:2.1.8]
	at net.minecraftforge.bootstrap.Bootstrap.start(Bootstrap.java:53) [bootstrap-2.1.8.jar:2.1.8]
	at net.minecraftforge.bootstrap.ForgeBootstrap.main(ForgeBootstrap.java:19) [bootstrap-2.1.8.jar:2.1.8]
	at java.base/jdk.internal.reflect.DirectMethodHandleAccessor.invoke(DirectMethodHandleAccessor.java:103) ~[?:?]
	at java.base/java.lang.reflect.Method.invoke(Method.java:580) ~[?:?]
	at io.github.zekerzhayard.forgewrapper.installer.Main.main(Main.java:67) [ForgeWrapper-prism-2025-12-07.jar:prism-2025-12-07]
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70) [NewLaunch.jar:?]
Caused by: java.lang.IllegalStateException: Cannot get config value before config is loaded.
	at SECURE-BOOTSTRAP/com.google.common@33.3.1-jre/com.google.common.base.Preconditions.checkState(Preconditions.java:513) ~[guava-33.3.1-jre.jar!/:?]
	at TRANSFORMER/forgeconfigapiport@21.10.1/net.neoforged.neoforge.common.ModConfigSpec$ConfigValue.getRaw(ModConfigSpec.java:1212) ~[ForgeConfigAPIPort-v21.10.1 mc1.21.10-Forge.jar!/:21.10.1]
	at TRANSFORMER/forgeconfigapiport@21.10.1/net.neoforged.neoforge.common.ModConfigSpec$ConfigValue.get(ModConfigSpec.java:1199) ~[ForgeConfigAPIPort-v21.10.1 mc1.21.10-Forge.jar!/:21.10.1]
	at TRANSFORMER/shouldersurfing@1.21.10-4.17.1/com.github.exopandora.shouldersurfing.config.Config$ClientConfig.getOffsetX(Config.java:795) ~[ShoulderSurfing-Forge-1.21.10-4.17.1.jar!/:1.21.10-4.17.1]
	at TRANSFORMER/shouldersurfing@1.21.10-4.17.1/com.github.exopandora.shouldersurfing.client.ShoulderSurfingCamera.init(ShoulderSurfingCamera.java:108) ~[ShoulderSurfing-Forge-1.21.10-4.17.1.jar!/:1.21.10-4.17.1]
	at TRANSFORMER/shouldersurfing@1.21.10-4.17.1/com.github.exopandora.shouldersurfing.client.ShoulderSurfingCamera.<init>(ShoulderSurfingCamera.java:54) ~[ShoulderSurfing-Forge-1.21.10-4.17.1.jar!/:1.21.10-4.17.1]
	at TRANSFORMER/shouldersurfing@1.21.10-4.17.1/com.github.exopandora.shouldersurfing.client.ShoulderSurfingImpl.<init>(ShoulderSurfingImpl.java:29) ~[ShoulderSurfing-Forge-1.21.10-4.17.1.jar!/:1.21.10-4.17.1]
	at java.base/jdk.internal.reflect.DirectConstructorHandleAccessor.newInstance(DirectConstructorHandleAccessor.java:62) ~[?:?]
	at java.base/java.lang.reflect.Constructor.newInstanceWithCaller(Constructor.java:502) ~[?:?]
	at java.base/java.lang.reflect.Constructor.newInstance(Constructor.java:486) ~[?:?]
	at java.base/java.util.ServiceLoader$ProviderImpl.newInstance(ServiceLoader.java:789) ~[?:?]
	... 31 more
Transformer Audit:
  com.github.exopandora.shouldersurfing.api.client.ShoulderSurfing
    REASON: classloading
  com.github.exopandora.shouldersurfing.client.ShoulderSurfingCamera
    REASON: mixin
    REASON: classloading
  com.github.exopandora.shouldersurfing.client.ShoulderSurfingImpl
    REASON: mixin
    REASON: classloading
  com.github.exopandora.shouldersurfing.config.Config$ClientConfig
    REASON: mixin
    REASON: classloading
  net.minecraft.client.Minecraft
    REASON: mixin
    PLUGIN: accesstransformer:BEFORE
    REASON: classloading
    PLUGIN: accesstransformer:BEFORE
    PLUGIN: mixin:APP:libbamboo.mixins.json:MinecraftClientAccessor
    PLUGIN: mixin:APP:libbamboo.mixins.json:MinecraftClientMixin
    PLUGIN: mixin:APP:mixins/common/nochatreports.mixins.json:client.MixinMinecraft
    PLUGIN: mixin:AFTER
  net.minecraft.client.main.Main
    REASON: classloading
  net.minecraft.client.renderer.GameRenderer
    REASON: classloading
    PLUGIN: mixin:APP:shouldersurfing.common.mixins.json:GameRendererAccessor
    PLUGIN: mixin:APP:shouldersurfing.common.mixins.json:MixinGameRenderer
    PLUGIN: mixin:AFTER
  net.neoforged.neoforge.common.ModConfigSpec$ConfigValue
    REASON: classloading


A detailed walkthrough of the error, its code path and all known details is as follows:
---------------------------------------------------------------------------------------

-- Head --
Thread: Render thread
"#;
        let crash_report = CrashReport::parse(crash_report_fragment).expect("Failed to parse crash report");

		let mods = indexed_header.get_mods().unwrap();
        let issue = check_mods_in_stacktrace_info(&mods, &crash_report).expect("Failed to find issue");
		let Issue::ModsFoundInStacktraceInfo(mods) = issue else { panic!("Not the right issue"); };
		assert_eq!(mods.len(), 2);
		assert!(mods.contains("ShoulderSurfing-Forge-1.21.10-4.17.1.jar"))
    }
}