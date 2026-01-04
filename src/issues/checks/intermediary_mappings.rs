use crate::{entries::entry::LogEntry, issues::issue::Issue};

fn intermediary_mappings(entry: &LogEntry) -> Option<Issue> {
    entry.contents.starts_with("Mapping source name conflicts detected:").then_some(Issue::WrongIntermediaryMappingsVersion)
}

#[cfg(test)]
mod tests {
    use crate::entries::entry::LogEntry;

    use super::*;

    #[test]
    fn matches_intermediary_mappings() {
        let text = r#"Exception caught from launcher
java.lang.RuntimeException: Unfixable conflicts
	at net.fabricmc.loader.impl.lib.tinyremapper.TinyRemapper.handleConflicts(TinyRemapper.java:922)
	at net.fabricmc.loader.impl.lib.tinyremapper.TinyRemapper.propagate(TinyRemapper.java:821)
	at net.fabricmc.loader.impl.lib.tinyremapper.TinyRemapper.mrjRefresh(TinyRemapper.java:1101)
	at net.fabricmc.loader.impl.lib.tinyremapper.TinyRemapper.apply(TinyRemapper.java:953)
	at net.fabricmc.loader.impl.game.GameProviderHelper.deobfuscate0(GameProviderHelper.java:365)
	at net.fabricmc.loader.impl.game.GameProviderHelper.deobfuscate(GameProviderHelper.java:296)
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.initialize(MinecraftGameProvider.java:364)
	at net.fabricmc.loader.impl.launch.knot.Knot.init(Knot.java:141)
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:66)
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
Exiting with ERROR
[20:36:34] [INFO] [FabricLoader/GameProvider]: Loading Minecraft 1.21.10 with Fabric Loader 0.18.1
[20:36:34] [WARN] [FabricLoader/GameRemap]: Incomplete remapped file found! This means that the remapping process failed on the previous launch. If this persists, make sure to let us at Fabric know!
[20:36:34] [INFO] [FabricLoader/GameRemap]: Fabric is preparing JARs on first launch, this may take a few seconds...
[20:36:36] [WARN] [FabricLoader/GameRemap]: Mapping source name conflicts detected:
[20:36:36] [WARN] [FabricLoader/GameRemap]: ehp METHOD b (()I) -> [ehp/method_12031, ego/method_12197]
"#;
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines()).collect();
        let issue = entries.iter().filter_map(|e| intermediary_mappings(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::WrongIntermediaryMappingsVersion);
    }
}