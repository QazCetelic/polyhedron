use crate::issues::issue::Issue;

fn lwjgl_2_java_9(text: &str) -> Option<Issue> {
    text.contains("check_match: Assertion `version->filename == NULL || ! _dl_name_match_p (version->filename, map)' failed!").then_some(Issue::Lwjgl2Java9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_lwjgl_2_java_9() {
        // Text starts after header
        let text = "[17:23:00] [Client-Main] 24 Achievements
[17:23:00] [Client-Main] Resolution: 854 x 480
[17:23:00] [Client-Main] Java Version: 21.0.4
[17:23:00] [Client-Main] java.lang.ArrayIndexOutOfBoundsException: Index 0 out of bounds for length 0
[17:23:00] [Client-Main]     at net.minecraft.client.Minecraft.printWrongJavaVersionInfo(Minecraft.java:549)
[17:23:00] [Client-Main]     at net.minecraft.client.Minecraft.startGame(Minecraft.java:355)
[17:23:00] [Client-Main]     at net.minecraft.client.Minecraft.run(Minecraft.java:796)
[17:23:00] [Client-Main]     at java.base/java.lang.Thread.run(Thread.java:1583)
Inconsistency detected by ld.so: dl-lookup.c: 107: check_match: Assertion `version->filename == NULL || ! _dl_name_match_p (version->filename, map)' failed!
Process exited with code 127.
Clipboard copy at: 27 Sep 2024 17:30:12 +0530
Reply
Forward
More
[14:01]Friday 27 September 2024 at 14:01
";
        let issue = lwjgl_2_java_9(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::Lwjgl2Java9);
    }
}