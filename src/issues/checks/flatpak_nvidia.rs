use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub(crate) fn flatpak_nvidia(entry: &LogEntry) -> Option<Issue> {
    if entry.contents.contains("org.lwjgl.LWJGLException: Could not choose GLX13 config") || entry.contents.contains("GLX: Failed to find a suitable GLXFBConfig") {
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
        let text = r#"[22:58:35] [Client thread/INFO]: LWJGL Version: 2.9.4
[22:58:35] [Client thread/ERROR]: Couldn't set pixel format
org.lwjgl.LWJGLException: Could not choose GLX13 config
	at org.lwjgl.opengl.LinuxDisplayPeerInfo.initDefaultPeerInfo(Native Method) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.LinuxDisplayPeerInfo.<init>(LinuxDisplayPeerInfo.java:61) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.LinuxDisplay.createPeerInfo(LinuxDisplay.java:828) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.DrawableGL.setPixelFormat(DrawableGL.java:61) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.Display.create(Display.java:846) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.Display.create(Display.java:757) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at net.minecraftforge.client.ForgeHooksClient.createDisplay(ForgeHooksClient.java:327) ~[ForgeHooksClient.class:?]
	at net.minecraft.client.Minecraft.func_71384_a(Minecraft.java:432) [bao.class:?]
	at net.minecraft.client.Minecraft.func_99999_d(Minecraft.java:7099) [bao.class:?]
	at net.minecraft.client.main.Main.main(SourceFile:148) [Main.class:?]
	at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method) ~[?:1.8.0_202]
	at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62) ~[?:1.8.0_202]
	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43) ~[?:1.8.0_202]
	at java.lang.reflect.Method.invoke(Method.java:498) ~[?:1.8.0_202]
	at net.minecraft.launchwrapper.Launch.launch(Launch.java:135) [launchwrapper-1.12.jar:?]
	at net.minecraft.launchwrapper.Launch.main(Launch.java:28) [launchwrapper-1.12.jar:?]
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70) [NewLaunch.jar:?]

"#;
		let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
		let issue = entries.iter().filter_map(|e| flatpak_nvidia(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::OutdatedFlatpakNvidiaDriver);
    }
}