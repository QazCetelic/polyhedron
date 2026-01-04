use crate::issues::issue::Issue;

fn new_java_old_forge_legacy_java_fixer(entry_text: &str) -> Option<Issue> {
    entry_text.contains("[SEVERE] [ForgeModLoader] Unable to launch\njava.util.ConcurrentModificationException").then_some(Issue::NewJavaOldForgeLegacyJavaFixer)
}

fn new_java_old_forge_ignore_certificates(entry_text: &str) -> Option<Issue> {
    entry_text.contains("add the flag -Dfml.ignoreInvalidMinecraftCertificates=true to the 'JVM settings'").then_some(Issue::NewJavaOldForgeIgnoreCerts)
}   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_new_java_old_forge_legacy_java_fixer() {
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
        let issue = new_java_old_forge_legacy_java_fixer(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::NewJavaOldForgeLegacyJavaFixer);
    }

    #[test]
    fn matches_new_java_old_forge_ignore_certificates() {
        let text = r"2023-12-14 20:15:40 [SEVERE] [ForgeModLoader] The minecraft jar file:/C:/Users/********/AppData/Roaming/PrismLauncher/libraries/com/mojang/minecraft/1.6.4/minecraft-1.6.4-client.jar!/net/minecraft/client/ClientBrandRetriever.class appears to be corrupt! There has been CRITICAL TAMPERING WITH MINECRAFT, it is highly unlikely minecraft will work! STOP NOW, get a clean copy and try again!
2023-12-14 20:15:40 [SEVERE] [ForgeModLoader] For your safety, FML will not launch minecraft. You will need to fetch a clean version of the minecraft jar file
2023-12-14 20:15:40 [SEVERE] [ForgeModLoader] Technical information: The class net.minecraft.client.ClientBrandRetriever should have been associated with the minecraft jar file, and should have returned us a valid, intact minecraft jar location. This did not work. Either you have modified the minecraft jar file (if so run the forge installer again), or you are using a base editing jar that is changing this class (and likely others too). If you REALLY want to run minecraft in this configuration, add the flag -Dfml.ignoreInvalidMinecraftCertificates=true to the 'JVM settings' in your launcher profile.
";
        let issue = new_java_old_forge_ignore_certificates(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::NewJavaOldForgeIgnoreCerts);
    }
}