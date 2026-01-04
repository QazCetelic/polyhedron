use crate::issues::issue::Issue;

fn flatpak_nvidia(text: &str) -> Option<Issue> {
    if text.contains("org.lwjgl.LWJGLException: Could not choose GLX13 config") || text.contains("GLX: Failed to find a suitable GLXFBConfig") {
        Some(Issue::OutdatedFlatpakNvidiaDriver)
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_flatpak_nvidia() {
        let text = "---- Minecraft Crash Report ----
// Would you like a cupcake?

Time: 2025-09-16 22:58:36 CEST
Description: Initializing game

org.lwjgl.LWJGLException: Could not choose GLX13 config
    at org.lwjgl.opengl.LinuxDisplayPeerInfo.initDefaultPeerInfo(Native Method)
    at org.lwjgl.opengl.LinuxDisplayPeerInfo.<init>(LinuxDisplayPeerInfo.java:61)
    at org.lwjgl.opengl.LinuxDisplay.createPeerInfo(LinuxDisplay.java:828)
    at org.lwjgl.opengl.DrawableGL.setPixelFormat(DrawableGL.java:61)
    at org.lwjgl.opengl.Display.create(Display.java:846)
    at org.lwjgl.opengl.Display.create(Display.java:757)
    at org.lwjgl.opengl.Display.create(Display.java:739)
    at net.minecraft.client.Minecraft.func_71384_a(Minecraft.java:452)
    at net.minecraft.client.Minecraft.func_99999_d(Minecraft.java:7099)
    at net.minecraft.client.main.Main.main(SourceFile:148)
    at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
    at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)
    at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
    at java.lang.reflect.Method.invoke(Method.java:498)
    at net.minecraft.launchwrapper.Launch.launch(Launch.java:135)
    at net.minecraft.launchwrapper.Launch.main(Launch.java:28)
    at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
    at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
    at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)";

        let issue = flatpak_nvidia(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::OutdatedFlatpakNvidiaDriver);
    }
}