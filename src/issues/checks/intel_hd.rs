use crate::{header::extract::JavaVersionInfo, issues::issue::Issue};

// "On first- and second- generation Intel HD graphics chipsets, a Java version below 8u60 is needed on Windows 10" https://minecrafthopper.net/help/pixel-format-not-accelerated/

fn intel_hd(text: &str, java_version: Option<&JavaVersionInfo>) -> Option<Issue> {
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
    use super::*;

    #[test]
    fn matches_intel_hd() {
        let text = "---- Minecraft Crash Report ----
// You're mean.

Time: 2/15/23 10:46 PM
Description: Initializing game

org.lwjgl.LWJGLException: Pixel format not accelerated
	at org.lwjgl.opengl.WindowsPeerInfo.nChoosePixelFormat(Native Method)
	at org.lwjgl.opengl.WindowsPeerInfo.choosePixelFormat(WindowsPeerInfo.java:52)
	at org.lwjgl.opengl.WindowsDisplay.createWindow(WindowsDisplay.java:247)
	at org.lwjgl.opengl.Display.createWindow(Display.java:306)
	at org.lwjgl.opengl.Display.create(Display.java:848)
	at org.lwjgl.opengl.Display.create(Display.java:757)
	at org.lwjgl.opengl.Display.create(Display.java:739)
	at ave.ap(SourceFile:534)
	at ave.am(SourceFile:363)
	at ave.a(SourceFile:310)
	at net.minecraft.client.main.Main.main(SourceFile:124)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:88)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:126)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:71)

";

        let issue = intel_hd(&text, None).expect("Failed to determine issue");
        assert_eq!(issue, Issue::IntelHd);
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