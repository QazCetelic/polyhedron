use crate::{header::{extract::LibraryInfo, index::IndexedLogHeader}, issues::issue::Issue};

fn missing_libraries(libraries: &[LibraryInfo]) -> Option<Issue> {
    let missing_libs: Vec<String> = libraries.iter()
        .filter(|lib| lib.missing)
        .map(|lib| lib.name.to_string())
        .collect();
    (!missing_libs.is_empty()).then_some(Issue::MissingLibraries(missing_libs))
}

pub(crate) fn missing_libraries_header(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    let mut all_libs = Vec::new();
    if let Some(mut libs) = header.get_libraries() {
        all_libs.append(&mut libs);
    }
    if let Some(mut native_libs) = header.get_native_libraries() {
        all_libs.append(&mut native_libs);
    }
    missing_libraries(&all_libs)
}

#[cfg(test)]
mod tests {
    use crate::header::index::IndexedLogHeader;

    use super::*;

    #[test]
    fn matches_missing_libraries() {
        let header_fragment = r"
Libraries:
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-glfw-natives-windows-arm64/3.3.1/lwjgl-glfw-natives-windows-arm64-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-glfw-natives-windows-x86/3.3.1/lwjgl-glfw-natives-windows-x86-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-glfw-natives-windows/3.3.1/lwjgl-glfw-natives-windows-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-jemalloc-natives-windows-arm64/3.3.1/lwjgl-jemalloc-natives-windows-arm64-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-jemalloc-natives-windows-x86/3.3.1/lwjgl-jemalloc-natives-windows-x86-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-jemalloc-natives-windows/3.3.1/lwjgl-jemalloc-natives-windows-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-jemalloc/3.3.1/lwjgl-jemalloc-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-natives-windows-arm64/3.3.1/lwjgl-natives-windows-arm64-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-natives-windows-x86/3.3.1/lwjgl-natives-windows-x86-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-natives-windows/3.3.1/lwjgl-natives-windows-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-openal-natives-windows-arm64/3.3.1/lwjgl-openal-natives-windows-arm64-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-openal-natives-windows-x86/3.3.1/lwjgl-openal-natives-windows-x86-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-openal-natives-windows/3.3.1/lwjgl-openal-natives-windows-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-openal/3.3.1/lwjgl-openal-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-opengl-natives-windows-arm64/3.3.1/lwjgl-opengl-natives-windows-arm64-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-opengl-natives-windows-x86/3.3.1/lwjgl-opengl-natives-windows-x86-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-opengl-natives-windows/3.3.1/lwjgl-opengl-natives-windows-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-opengl/3.3.1/lwjgl-opengl-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-stb-natives-windows-arm64/3.3.1/lwjgl-stb-natives-windows-arm64-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-stb-natives-windows-x86/3.3.1/lwjgl-stb-natives-windows-x86-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-stb-natives-windows/3.3.1/lwjgl-stb-natives-windows-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-stb/3.3.1/lwjgl-stb-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-tinyfd-natives-windows-arm64/3.3.1/lwjgl-tinyfd-natives-windows-arm64-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-tinyfd-natives-windows-x86/3.3.1/lwjgl-tinyfd-natives-windows-x86-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-tinyfd-natives-windows/3.3.1/lwjgl-tinyfd-natives-windows-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-tinyfd/3.3.1/lwjgl-tinyfd-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/github/oshi/oshi-core/6.2.2/oshi-core-6.2.2.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/google/code/gson/gson/2.10/gson-2.10.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/google/guava/failureaccess/1.0.1/failureaccess-1.0.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/google/guava/guava/31.1-jre/guava-31.1-jre.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/ibm/icu/icu4j/71.1/icu4j-71.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/mojang/authlib/4.0.43/authlib-4.0.43.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/mojang/blocklist/1.0.10/blocklist-1.0.10.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/mojang/brigadier/1.1.8/brigadier-1.1.8.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/mojang/datafixerupper/6.0.8/datafixerupper-6.0.8.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/mojang/logging/1.1.1/logging-1.1.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/mojang/patchy/2.2.10/patchy-2.2.10.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/mojang/text2speech/1.17.9/text2speech-1.17.9.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/commons-codec/commons-codec/1.15/commons-codec-1.15.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/commons-io/commons-io/2.11.0/commons-io-2.11.0.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/commons-logging/commons-logging/1.2/commons-logging-1.2.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/netty/netty-buffer/4.1.82.Final/netty-buffer-4.1.82.Final.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/netty/netty-codec/4.1.82.Final/netty-codec-4.1.82.Final.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/netty/netty-common/4.1.82.Final/netty-common-4.1.82.Final.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/netty/netty-handler/4.1.82.Final/netty-handler-4.1.82.Final.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/netty/netty-resolver/4.1.82.Final/netty-resolver-4.1.82.Final.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/netty/netty-transport-classes-epoll/4.1.82.Final/netty-transport-classes-epoll-4.1.82.Final.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/netty/netty-transport-native-unix-common/4.1.82.Final/netty-transport-native-unix-common-4.1.82.Final.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/netty/netty-transport/4.1.82.Final/netty-transport-4.1.82.Final.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/it/unimi/dsi/fastutil/8.5.9/fastutil-8.5.9.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/java/dev/jna/jna-platform/5.12.1/jna-platform-5.12.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/java/dev/jna/jna/5.12.1/jna-5.12.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/sf/jopt-simple/jopt-simple/5.0.4/jopt-simple-5.0.4.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/apache/commons/commons-compress/1.21/commons-compress-1.21.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/apache/commons/commons-lang3/3.12.0/commons-lang3-3.12.0.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/apache/httpcomponents/httpclient/4.5.13/httpclient-4.5.13.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/apache/httpcomponents/httpcore/4.4.15/httpcore-4.4.15.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/apache/logging/log4j/log4j-api/2.19.0/log4j-api-2.19.0.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/apache/logging/log4j/log4j-core/2.19.0/log4j-core-2.19.0.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/apache/logging/log4j/log4j-slf4j2-impl/2.19.0/log4j-slf4j2-impl-2.19.0.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/joml/joml/1.10.5/joml-1.10.5.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/slf4j/slf4j-api/2.0.1/slf4j-api-2.0.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/github/zekerzhayard/ForgeWrapper/prism-2025-12-07/ForgeWrapper-prism-2025-12-07.jar (missing)
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/cpw/mods/securejarhandler/2.1.10/securejarhandler-2.1.10.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/ow2/asm/asm/9.8/asm-9.8.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/ow2/asm/asm-commons/9.8/asm-commons-9.8.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/ow2/asm/asm-tree/9.8/asm-tree-9.8.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/ow2/asm/asm-util/9.8/asm-util-9.8.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/ow2/asm/asm-analysis/9.8/asm-analysis-9.8.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/accesstransformers/8.0.4/accesstransformers-8.0.4.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/antlr/antlr4-runtime/4.9.1/antlr4-runtime-4.9.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/eventbus/6.0.5/eventbus-6.0.5.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/forgespi/7.0.1/forgespi-7.0.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/coremods/5.2.4/coremods-5.2.4.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/cpw/mods/modlauncher/10.0.9/modlauncher-10.0.9.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/unsafe/0.2.0/unsafe-0.2.0.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/mergetool/1.1.5/mergetool-1.1.5-api.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/electronwill/night-config/core/3.6.4/core-3.6.4.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/electronwill/night-config/toml/3.6.4/toml-3.6.4.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/apache/maven/maven-artifact/3.8.5/maven-artifact-3.8.5.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/jodah/typetools/0.6.3/typetools-0.6.3.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecrell/terminalconsoleappender/1.2.0/terminalconsoleappender-1.2.0.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/jline/jline-reader/3.12.1/jline-reader-3.12.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/jline/jline-terminal/3.12.1/jline-terminal-3.12.1.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/spongepowered/mixin/0.8.5/mixin-0.8.5.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/org/openjdk/nashorn/nashorn-core/15.4/nashorn-core-15.4.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/JarJarSelector/0.3.19/JarJarSelector-0.3.19.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/JarJarMetadata/0.3.19/JarJarMetadata-0.3.19.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/cpw/mods/bootstraplauncher/1.1.2/bootstraplauncher-1.1.2.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/JarJarFileSystems/0.3.19/JarJarFileSystems-0.3.19.jar
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/fmlloader/1.20.1-47.4.10/fmlloader-1.20.1-47.4.10.jar (missing)
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/fmlearlydisplay/1.20.1-47.4.10/fmlearlydisplay-1.20.1-47.4.10.jar (missing)
  C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/com/mojang/minecraft/1.20.1/minecraft-1.20.1-client.jar

Native libraries:

Mods:
";
        let indexed = IndexedLogHeader::index_header(header_fragment);
        let issue = missing_libraries_header(&indexed).expect("Failed to determine issue");
        let Issue::MissingLibraries(libs) = issue else { panic!("Not MissingLibraries issue"); };
        assert_eq!(libs, vec![
            "C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/io/github/zekerzhayard/ForgeWrapper/prism-2025-12-07/ForgeWrapper-prism-2025-12-07.jar",
            "C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/fmlloader/1.20.1-47.4.10/fmlloader-1.20.1-47.4.10.jar",
            "C:/Users/Ultra/AppData/Roaming/PrismLauncher/libraries/net/minecraftforge/fmlearlydisplay/1.20.1-47.4.10/fmlearlydisplay-1.20.1-47.4.10.jar"
        ]);
    }
}