use crate::issues::issue::Issue;

fn missing_xrandr(text: &str) -> Option<Issue> {
   text.contains("at org.lwjgl.opengl.LinuxDisplay.getAvailableDisplayModes").then_some(Issue::MissingXrandr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_missing_xrandr() {
        let text = r#"[22:31:48] [main/INFO] [LaunchWrapper]: Launching wrapped minecraft {net.minecraft.client.main.Main}
[22:31:49] [Client thread/INFO]: Setting user: Wallemajak
[22:31:53] [Client thread/INFO]: LWJGL Version: 2.9.4
[22:31:53] [Client thread/INFO] [STDOUT]: [net.minecraft.init.Bootstrap:func_179870_a:529]: ---- Minecraft Crash Report ----
// Uh... Did I do that?

Time: 7/15/25 10:31 PM
Description: Initializing game

java.lang.ExceptionInInitializerError
	at net.minecraft.client.Minecraft.func_175594_ao(Minecraft.java:615)
	at net.minecraft.client.Minecraft.func_71384_a(Minecraft.java:406)
	at net.minecraft.client.Minecraft.func_99999_d(Minecraft.java:329)
	at net.minecraft.client.main.Main.main(SourceFile:124)
	at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
	at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)
	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
	at java.lang.reflect.Method.invoke(Method.java:498)
	at net.minecraft.launchwrapper.Launch.launch(Launch.java:135)
	at net.minecraft.launchwrapper.Launch.main(Launch.java:28)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
Caused by: java.lang.ArrayIndexOutOfBoundsException: 0
	at org.lwjgl.opengl.LinuxDisplay.getAvailableDisplayModes(LinuxDisplay.java:951)
	at org.lwjgl.opengl.LinuxDisplay.init(LinuxDisplay.java:738)
	at org.lwjgl.opengl.Display.<clinit>(Display.java:138)
	... 13 more
"#;
        let issue = missing_xrandr(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::MissingXrandr);
    }
}