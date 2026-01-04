use crate::issues::issue::Issue;

fn modern_java_old_forge(entry_text: &str) -> Option<Issue> {
    entry_text.contains("[SEVERE] [ForgeModLoader] Unable to launch\njava.util.ConcurrentModificationException").then_some(Issue::ModernJavaOldForge)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_modern_java_old_forge() {
        let text = r"2023-02-13 00:00:23 [SEVERE] [ForgeModLoader] Unable to launch
java.util.ConcurrentModificationException
    at java.util.ArrayList$Itr.checkForComodification(ArrayList.java:911)
    at java.util.ArrayList$Itr.remove(ArrayList.java:875)
    at net.minecraft.launchwrapper.Launch.launch(Launch.java:114)
    at net.minecraft.launchwrapper.Launch.main(Launch.java:27)
    at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
    at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)
    at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
    at java.lang.reflect.Method.invoke(Method.java:498)
    at org.prismlauncher.impl.OneSixLauncher.invokeMain(OneSixLauncher.java:104)
    at org.prismlauncher.impl.OneSixLauncher.launchWithMainClass(OneSixLauncher.java:176)
    at org.prismlauncher.impl.OneSixLauncher.launch(OneSixLauncher.java:186)
    at org.prismlauncher.EntryPoint.listen(EntryPoint.java:144)
    at org.prismlauncher.EntryPoint.main(EntryPoint.java:74)
";
        let issue = modern_java_old_forge(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::ModernJavaOldForge);
    }
}