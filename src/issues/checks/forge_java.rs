use crate::issues::issue::Issue;

fn forge_java(text: &str) -> Option<Issue> {
    if text.contains("java.lang.NoSuchMethodError: sun.security.util.ManifestEntryVerifier.<init>(Ljava/util/jar/Manifest;)V") {
        Some(Issue::ForgeJava)
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_forge_java() {
        let text = "[21:05:17] [main/ERROR] [mixin/]: Mixin config antiqueatlas.mixins.json does not specify \"minVersion\" property
Exception caught from launcher
java.lang.reflect.InvocationTargetException
	at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
	at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)
	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
	at java.lang.reflect.Method.invoke(Method.java:498)
	at io.github.zekerzhayard.forgewrapper.installer.Main.main(Main.java:66)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:87)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:130)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
Caused by: java.lang.NoSuchMethodError: sun.security.util.ManifestEntryVerifier.<init>(Ljava/util/jar/Manifest;)V
	at cpw.mods.modlauncher.SecureJarHandler.createCodeSource(SecureJarHandler.java:66)
	at cpw.mods.modlauncher.TransformingClassLoader$DelegatedClassLoader.findClass(TransformingClassLoader.java:275)
	at cpw.mods.modlauncher.TransformingClassLoader.loadClass(TransformingClassLoader.java:136)
	at cpw.mods.modlauncher.TransformingClassLoader.loadClass(TransformingClassLoader.java:98)
	at java.lang.ClassLoader.loadClass(ClassLoader.java:351)
	at java.lang.Class.forName0(Native Method)
	at java.lang.Class.forName(Class.java:348)
	at org.spongepowered.asm.service.modlauncher.ModLauncherClassProvider.findClass(ModLauncherClassProvider.java:67)
	at org.spongepowered.asm.launch.platform.MixinConnectorManager.loadConnectors(MixinConnectorManager.java:70)
	at org.spongepowered.asm.launch.platform.MixinConnectorManager.inject(MixinConnectorManager.java:59)
	at org.spongepowered.asm.launch.platform.MixinPlatformManager.inject(MixinPlatformManager.java:196)
	at org.spongepowered.asm.launch.MixinBootstrap.inject(MixinBootstrap.java:202)
	at org.spongepowered.asm.launch.MixinLaunchPluginLegacy.initializeLaunch(MixinLaunchPluginLegacy.java:201)
	at org.spongepowered.asm.launch.MixinLaunchPluginLegacy.initializeLaunch(MixinLaunchPluginLegacy.java:195)
	at cpw.mods.modlauncher.LaunchPluginHandler.lambda$announceLaunch$9(LaunchPluginHandler.java:97)
	at java.util.HashMap.forEach(HashMap.java:1290)
	at cpw.mods.modlauncher.LaunchPluginHandler.announceLaunch(LaunchPluginHandler.java:97)
	at cpw.mods.modlauncher.LaunchServiceHandler.launch(LaunchServiceHandler.java:52)
	at cpw.mods.modlauncher.LaunchServiceHandler.launch(LaunchServiceHandler.java:72)
	at cpw.mods.modlauncher.Launcher.run(Launcher.java:82)
	at cpw.mods.modlauncher.Launcher.main(Launcher.java:66)
	... 8 more
Exiting with ERROR";

        let issue = forge_java(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::ForgeJava);
    }
}