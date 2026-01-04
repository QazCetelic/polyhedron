use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub(crate) fn missing_indium(entry: &LogEntry) -> Option<Issue> {
    entry.contents.contains("Cannot invoke \"net.fabricmc.fabric.api.renderer.v1.Renderer.meshBuilder()\"").then_some(Issue::MissingIndium)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_missing_indium() {
        let text = r#"[21:54:08] [Render thread/INFO]: Stopping worker threads
[21:54:09] [Render thread/ERROR]: Reported exception thrown!
net.minecraft.class_148: Rendering entity in world
	at net.minecraft.class_898.method_3954(class_898.java:176) ~[client-intermediary.jar:?]
	at net.minecraft.class_761.method_22977(class_761.java:1574) ~[client-intermediary.jar:?]
	at net.minecraft.class_761.method_22710(class_761.java:1324) ~[client-intermediary.jar:?]
	at net.minecraft.class_757.method_3188(class_757.java:1110) ~[client-intermediary.jar:?]
	at net.minecraft.class_757.method_3192(class_757.java:880) ~[client-intermediary.jar:?]
	at net.minecraft.class_310.method_1523(class_310.java:1219) ~[client-intermediary.jar:?]
	at net.minecraft.class_310.method_1514(class_310.java:802) ~[client-intermediary.jar:?]
	at net.minecraft.client.main.Main.main(Main.java:250) ~[minecraft-1.20.1-client.jar:?]
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:506) ~[fabric-loader-0.17.2.jar:?]
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72) ~[fabric-loader-0.17.2.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23) ~[fabric-loader-0.17.2.jar:?]
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105) ~[NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129) ~[NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70) ~[NewLaunch.jar:?]
Caused by: java.lang.NullPointerException: Cannot invoke "net.fabricmc.fabric.api.renderer.v1.Renderer.meshBuilder()" because the return value of "net.fabricmc.fabric.api.renderer.v1.RendererAccess.getRenderer()" is null
	at me.pepperbell.continuity.impl.client.ProcessingContextImpl.<init>(ProcessingContextImpl.java:20) ~[continuity-3.0.0 1.20.1.jar:?]
	at me.pepperbell.continuity.client.model.CtmBakedModel$CtmQuadTransform.<init>(CtmBakedModel.java:104) ~[continuity-3.0.0 1.20.1.jar:?]
	at me.pepperbell.continuity.client.model.ModelObjectsContainer.<init>(ModelObjectsContainer.java:10) ~[continuity-3.0.0 1.20.1.jar:?]
	at java.lang.ThreadLocal$SuppliedThreadLocal.initialValue(ThreadLocal.java:305) ~[?:?]
	at java.lang.ThreadLocal.setInitialValue(ThreadLocal.java:195) ~[?:?]
	at java.lang.ThreadLocal.get(ThreadLocal.java:172) ~[?:?]
	at me.pepperbell.continuity.client.model.ModelObjectsContainer.get(ModelObjectsContainer.java:18) ~[continuity-3.0.0 1.20.1.jar:?]
	at me.pepperbell.continuity.impl.client.ContinuityFeatureStatesImpl.get(ContinuityFeatureStatesImpl.java:16) ~[continuity-3.0.0 1.20.1.jar:?]
	at me.pepperbell.continuity.api.client.ContinuityFeatureStates.get(ContinuityFeatureStates.java:10) ~[continuity-3.0.0 1.20.1.jar:?]
	at net.minecraft.class_901.handler$zea000$continuity$beforeRenderModel(class_901.java:515) ~[client-intermediary.jar:?]
	at net.minecraft.class_901.method_3965(class_901.java:45) ~[client-intermediary.jar:?]
	at net.minecraft.class_901.method_3936(class_901.java:17) ~[client-intermediary.jar:?]
	at net.minecraft.class_898.method_3954(class_898.java:145) ~[client-intermediary.jar:?]
	... 13 more
"#;
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| missing_indium(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::MissingIndium);
    }
}