use crate::issues::issue::Issue;

pub(crate) fn oom(text: &str) -> Option<Issue> {
    // # There is insufficient memory for the Java Runtime Environment to continue.
    if text.contains("java.lang.OutOfMemoryError") {
        return Some(Issue::Oom)
    }
    if text.contains("# There is insufficient memory for the Java Runtime Environment to continue.") {
        return Some(Issue::Oom)
    }
    return None;
}

// According to: https://minecrafthopper.net/help/exit-code/code-805306369/
pub(crate) fn oom_exit_code(exit_code: i32) -> Option<Issue> {
    (exit_code == -805306369).then_some(Issue::Oom)
}

#[cfg(test)]
mod tests {
    use crate::parse::exit_code::extract_exit_code;

    use super::*;

    #[test]
    fn java_exception() {
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
        let issue = oom(text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::Oom);
    }

    #[test]
    fn insufficient_memory_jre() {
        let text = r#"  Patching net/minecraft/world/item/Items$1 1/1
OpenJDK 64-Bit Server VM warning: INFO: os::commit_memory(0x0000000795800000, 721420288, 0) failed; error='The paging file is too small for this operation to complete' (DOS error/errno=1455)
#
# There is insufficient memory for the Java Runtime Environment to continue.
# Native memory allocation (mmap) failed to map 721420288 bytes. Error detail: G1 virtual space
# An error report file with more information is saved as:
# C:\Users\********\AppData\Roaming\PrismLauncher\instances\Mijoma's Additional Additions\minecraft\hs_err_pid104756.log"#;
        let issue = oom(text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::Oom);
    }

    #[test]
    fn exit_code() {
        let text = r#"[00:46:00] [MicrophoneThread/INFO] [voicechat/]: [voicechat] Connection timeout
[00:46:00] [VoiceChatPacketProcessingThread/INFO] [voicechat/]: [voicechat] Reconnecting player DapperRitten
[00:46:03] [VoiceChatPacketProcessingThread/INFO] [voicechat/]: [voicechat] Sent secret to DapperRitten
[00:46:05] [MicrophoneThread/INFO] [voicechat/]: [voicechat] Stopping microphone thread
Process crashed with exitcode -805306369.
Log upload triggered at: 19 Nov 2025 00:48:09  0000"#;
        let (_, exit_code) = extract_exit_code(text).expect("Failed to extract exit code");
        let issue = oom_exit_code(exit_code).expect("Failed to determine issue");
        assert_eq!(issue, Issue::Oom);
    }
}