use crate::{header::extract::JavaVersionInfo, issues::issue::Issue};

// "On first- and second- generation Intel HD graphics chipsets, a Java version below 8u60 is needed on Windows 10" https://minecrafthopper.net/help/pixel-format-not-accelerated/

pub(crate) fn intel_hd(text: &str, java_version: Option<&JavaVersionInfo>) -> Option<Issue> {
    // Prism Launcher recommended Java version for Intel HD 2000/3000 on Windows 10 https://prismlauncher.org/wiki/getting-started/installing-java/#a-note-about-intel-hd-20003000-on-windows-10
    let is_not_recommended_version = java_version.map(|i| i.version != "1.8.0_51");
    if text.contains("org.lwjgl.LWJGLException: Pixel format not accelerated") && is_not_recommended_version.unwrap_or(true) {
        Some(Issue::IntelHd)
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::header::index::IndexedLogHeader;

    use super::*;

    #[test]
    fn without_recommended_java() {
        let text = r#"
Minecraft folder is:
C:/Users/********/AppData/Roaming/PrismLauncher/instances/1.8.9/minecraft


Java path is:
C:/Users/********/AppData/Roaming/PrismLauncher/java/eclipse_temurin_jre8.0.462 8/bin/javaw.exe


Checking Java version...
Java is version 1.8.0_462, using 64 (amd64) architecture, from Temurin.


Main Class:
  net.minecraft.launchwrapper.Launch

Native path:
  C:/Users/********/AppData/Roaming/PrismLauncher/instances/1.8.9/natives

...

[16:45:40] [Client thread/INFO]: LWJGL Version: 2.9.4
[16:45:40] [Client thread/ERROR]: Couldn't set pixel format
org.lwjgl.LWJGLException: Pixel format not accelerated
	at org.lwjgl.opengl.WindowsPeerInfo.nChoosePixelFormat(Native Method) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.WindowsPeerInfo.choosePixelFormat(WindowsPeerInfo.java:52) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.WindowsDisplay.createWindow(WindowsDisplay.java:247) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.Display.createWindow(Display.java:306) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.Display.create(Display.java:848) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.Display.create(Display.java:757) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at net.minecraft.client.Minecraft.func_175609_am(Minecraft.java:560) [ave.class:?]
	at net.minecraft.client.Minecraft.func_71384_a(Minecraft.java:408) [ave.class:?]
	at net.minecraft.client.Minecraft.func_99999_d(Minecraft.java:329) [ave.class:?]
	at net.minecraft.client.main.Main.main(SourceFile:124) [Main.class:?]
	at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method) ~[?:1.8.0_462]
	at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62) ~[?:1.8.0_462]
	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43) ~[?:1.8.0_462]
	at java.lang.reflect.Method.invoke(Method.java:498) ~[?:1.8.0_462]
	at net.minecraft.launchwrapper.Launch.launch(Launch.java:135) [launchwrapper-1.12.jar:?]
	at net.minecraft.launchwrapper.Launch.main(Launch.java:28) [launchwrapper-1.12.jar:?]
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70) [NewLaunch.jar:?]
[16:45:41] [Client thread/INFO] [STDOUT]: [net.minecraft.init.Bootstrap:func_179870_a:529]: ---- Minecraft Crash Report ----
// Who set us up the TNT?
"#;
        let indexed = IndexedLogHeader::index_header(text);
        let issue = intel_hd(&text, indexed.get_java_version().as_ref()).expect("Failed to determine issue");
        assert_eq!(issue, Issue::IntelHd);
    }

#[test]
    fn with_recommended_java() {
        let text = r#"
Minecraft folder is:
C:/Users/********/AppData/Roaming/PrismLauncher/instances/1.8.9/minecraft


Java path is:
C:/Users/********/AppData/Roaming/PrismLauncher/java/eclipse_temurin_jre8.0.462 8/bin/javaw.exe


Checking Java version...
Java is version 1.8.0_51, using 64 (amd64) architecture, from Temurin.


Main Class:
  net.minecraft.launchwrapper.Launch

Native path:
  C:/Users/********/AppData/Roaming/PrismLauncher/instances/1.8.9/natives

...

[16:45:40] [Client thread/INFO]: LWJGL Version: 2.9.4
[16:45:40] [Client thread/ERROR]: Couldn't set pixel format
org.lwjgl.LWJGLException: Pixel format not accelerated
	at org.lwjgl.opengl.WindowsPeerInfo.nChoosePixelFormat(Native Method) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.WindowsPeerInfo.choosePixelFormat(WindowsPeerInfo.java:52) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.WindowsDisplay.createWindow(WindowsDisplay.java:247) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.Display.createWindow(Display.java:306) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.Display.create(Display.java:848) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at org.lwjgl.opengl.Display.create(Display.java:757) ~[lwjgl-2.9.4-nightly-20150209.jar:?]
	at net.minecraft.client.Minecraft.func_175609_am(Minecraft.java:560) [ave.class:?]
	at net.minecraft.client.Minecraft.func_71384_a(Minecraft.java:408) [ave.class:?]
	at net.minecraft.client.Minecraft.func_99999_d(Minecraft.java:329) [ave.class:?]
	at net.minecraft.client.main.Main.main(SourceFile:124) [Main.class:?]
	at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method) ~[?:1.8.0_462]
	at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62) ~[?:1.8.0_462]
	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43) ~[?:1.8.0_462]
	at java.lang.reflect.Method.invoke(Method.java:498) ~[?:1.8.0_462]
	at net.minecraft.launchwrapper.Launch.launch(Launch.java:135) [launchwrapper-1.12.jar:?]
	at net.minecraft.launchwrapper.Launch.main(Launch.java:28) [launchwrapper-1.12.jar:?]
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70) [NewLaunch.jar:?]
[16:45:41] [Client thread/INFO] [STDOUT]: [net.minecraft.init.Bootstrap:func_179870_a:529]: ---- Minecraft Crash Report ----
// Who set us up the TNT?
"#;
        let indexed = IndexedLogHeader::index_header(text);
        let issue = intel_hd(&text, indexed.get_java_version().as_ref());
        assert!(issue.is_none());
    }

    #[test]
    fn recommended_version() {
        let text = "org.lwjgl.LWJGLException: Pixel format not accelerated";
        assert!(intel_hd(&text, None).map(|issue| issue == Issue::IntelHd).expect("Failed to determine issue"));
        let version_info = JavaVersionInfo {
            version: "1.8.0_51".to_string(),
            architecture: "64 (amd64)".to_string(),
            vendor: "Oracle Corporation".to_string(),
        };
        assert!(intel_hd(&text, Some(&version_info)).is_none());
    }
}