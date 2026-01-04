use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub(crate) fn oom(entry: &LogEntry) -> Option<Issue> {
    entry.contents.contains("java.lang.OutOfMemoryError").then_some(Issue::Oom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_oom() {
        // Text starts after header
        let text = "[20:00:45] [Render thread/ERROR]: Out of memory
java.lang.OutOfMemoryError: null
    at org.lwjgl.system.MemoryUtil.nmemReallocChecked(MemoryUtil.java:496) ~[lwjgl-3.3.1.jar:build 7]
    at org.lwjgl.system.MemoryUtil.memRealloc(MemoryUtil.java:529) ~[lwjgl-3.3.1.jar:build 7]
    at me.jellysquid.mods.sodium.client.render.vertex.type.ChunkVertexBufferBuilder.setBufferSize(ChunkVertexBufferBuilder.java:57) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.vertex.type.ChunkVertexBufferBuilder.start(ChunkVertexBufferBuilder.java:65) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.chunk.compile.ChunkBuildBuffers.init(ChunkBuildBuffers.java:61) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.chunk.tasks.ChunkRenderRebuildTask.performBuild(ChunkRenderRebuildTask.java:56) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.chunk.compile.ChunkBuilder.processJob(ChunkBuilder.java:286) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.chunk.compile.ChunkBuilder.stealTask(ChunkBuilder.java:246) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.common.util.collections.WorkStealingFutureDrain.findNext(WorkStealingFutureDrain.java:54) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.common.util.collections.WorkStealingFutureDrain.hasNext(WorkStealingFutureDrain.java:28) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.chunk.region.RenderRegionManager.setupUploadBatches(RenderRegionManager.java:123) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.chunk.region.RenderRegionManager.upload(RenderRegionManager.java:59) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.chunk.RenderSectionManager.updateChunks(RenderSectionManager.java:334) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.chunk.RenderSectionManager.updateChunks(RenderSectionManager.java:312) ~[transformed-mod-sodium.i0:0/:?]
    at me.jellysquid.mods.sodium.client.render.SodiumWorldRenderer.updateChunks(SodiumWorldRenderer.java:191) ~[transformed-mod-sodium.i0:0/:?]
    at net.minecraft.class_761.method_3273(class_761.java:12027) ~[transformed-mod-minecraft.i0:0/:?]
    at net.minecraft.class_761.method_22710(class_761.java:1240) ~[transformed-mod-minecraft.i0:0/:?]
    at net.minecraft.class_757.method_3188(class_757.java:1085) ~[transformed-mod-minecraft.i0:0/:?]
    at net.minecraft.class_757.method_3192(class_757.java:864) ~[transformed-mod-minecraft.i0:0/:?]
    at net.minecraft.class_310.method_1523(class_310.java:1193) ~[transformed-mod-minecraft.i0:0/:?]
    at net.minecraft.class_310.method_1514(class_310.java:781) ~[transformed-mod-minecraft.i0:0/:?]
    at net.minecraft.client.main.Main.method_44604(Main.java:244) ~[minecraft-1.19.3-client.jar:?]
    at net.minecraft.client.main.Main.main(Main.java:51) ~[minecraft-1.19.3-client.jar:?]
    at jdk.internal.reflect.DirectMethodHandleAccessor.invoke(DirectMethodHandleAccessor.java:104) ~[?:?]
    at java.lang.reflect.Method.invoke(Method.java:578) ~[?:?]
    at org.quiltmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:527) ~[quilt-loader-0.18.4-pre.1.jar:?]
    at org.quiltmc.loader.impl.launch.knot.Knot.launch(Knot.java:82) ~[quilt-loader-0.18.4-pre.1.jar:?]
    at org.quiltmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:28) ~[quilt-loader-0.18.4-pre.1.jar:?]
    at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:88) ~[NewLaunch.jar:?]
    at org.prismlauncher.EntryPoint.listen(EntryPoint.java:126) ~[NewLaunch.jar:?]
    at org.prismlauncher.EntryPoint.main(EntryPoint.java:71) ~[NewLaunch.jar:?]
";
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| oom(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::Oom);
    }
}