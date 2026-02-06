use crate::issues::issue::Issue;

pub struct FormattedIssueInfo {
    pub title: String,
    pub description: String,
}

pub fn format_issue(issue: &Issue) -> FormattedIssueInfo {
    match issue {
        Issue::OutdatedFlatpakNvidiaDriver => FormattedIssueInfo {
            title: "Outdated Flatpak Nvidia Driver".to_string(),
            description: "Please update your Nvidia driver that is bundled with Flatpak.".to_string(),
        },
        Issue::FabricInternalAccess => FormattedIssueInfo {
            title: "Fabric Internal Access".to_string(),
            description: "The mod you are using relies on Fabric internals. Consider using an updated mod.".to_string(),
        },
        Issue::LexforgeZlibng => FormattedIssueInfo {
            title: "Zlib-ng with Forge".to_string(),
            description: "Please ensure you are using the correct version of zlib-ng for Forge.".to_string(),
        },
        Issue::ForgeJava => FormattedIssueInfo {
            title: "Forge Java Bug".to_string(),
            description: "A bug exists in Forge with the Java version you are using. Please check compatibility.".to_string(),
        },
        Issue::IntelHd => FormattedIssueInfo {
            title: "Intel HD Graphics Issue".to_string(),
            description: "Check your Intel HD drivers or settings.".to_string(),
        },
        Issue::JavaOption(arg) => FormattedIssueInfo {
            title: "Wrong Java Option".to_string(),
            description: format!("You should remove the following Java option: {}", arg),
        },
        Issue::Lwjgl2JavaAbove8 => FormattedIssueInfo {
            title: "LWJGL 2 with Java Above 8".to_string(),
            description: "Please downgrade your Java version to 8 or use a newer LWJGL version.".to_string(),
        },
        Issue::MacOSNSInternal => FormattedIssueInfo {
            title: "MacOS NSInternalInconsistencyException".to_string(),
            description: "This is a known error with MacOS. Please seek solutions on forums.".to_string(),
        },
        Issue::Oom => FormattedIssueInfo {
            title: "Out of Memory".to_string(),
            description: "Your application has run out of memory. Consider adding more RAM.".to_string(),
        },
        Issue::Optifine => FormattedIssueInfo {
            title: "Optifine Enabled".to_string(),
            description: "Optifine is causing issues. Try disabling it.".to_string(),
        },
        Issue::OutdatedLauncher => FormattedIssueInfo {
            title: "Outdated Launcher".to_string(),
            description: "Please update your Minecraft launcher to the latest version.".to_string(),
        },
        Issue::NettyJavaAbove8 => FormattedIssueInfo {
            title: "Netty Java Compatibility".to_string(),
            description: "A version of Minecraft is using an outdated version of Netty that does not support Java 9+. Update Netty.".to_string(),
        },
        Issue::WrongJava(opt) => {
            let java_version = opt.map_or("unknown".to_string(), |v| v.to_string());
            FormattedIssueInfo {
                title: "Wrong Java Version".to_string(),
                description: format!("You should use Java version: {}", java_version),
            }
        },
        Issue::ForgeMissingDependencies => FormattedIssueInfo {
            title: "Missing Forge Dependencies".to_string(),
            description: "Some mod dependencies are missing. Please check the mod documentation.".to_string(),
        },
        Issue::NewJavaOldForgeLegacyJavaFixer => FormattedIssueInfo {
            title: "Legacy Java Fixer Required".to_string(),
            description: "You are using a modern Java version with an old Forge version. Please install LegacyJavaFixer.".to_string(),
        },
        Issue::LockedJars(items) => {
            let jars = items.join(", ");
            FormattedIssueInfo {
                title: "Locked Jars".to_string(),
                description: format!("The following JAR files are locked: {}", jars),
            }
        },
        Issue::MissingLibraries(items) => {
            let libraries = items.join(", ");
            FormattedIssueInfo {
                title: "Missing Libraries".to_string(),
                description: format!("The following libraries are missing: {}", libraries),
            }
        },
        Issue::MissingIndium => FormattedIssueInfo {
            title: "Missing Indium".to_string(),
            description: "Indium is a required mod and is currently missing. Please install it.".to_string(),
        },
        Issue::NoDiskSpace => FormattedIssueInfo {
            title: "No Disk Space".to_string(),
            description: "You are out of disk space. Please free up some space and try again.".to_string(),
        },
        Issue::Java32BitMemoryLimit => FormattedIssueInfo {
            title: "32 Bit Java Memory Limit".to_string(),
            description: "You are using a 32-bit version of Java which has a memory limit. Switch to a 64-bit version.".to_string(),
        },
        Issue::WrongIntermediaryMappingsVersion => FormattedIssueInfo {
            title: "Wrong Intermediary Mappings Version".to_string(),
            description: "Ensure that you are using the correct version of intermediary mappings.".to_string(),
        },
        Issue::NewJavaOldForgeIgnoreCerts => FormattedIssueInfo {
            title: "Disable Certificate Check".to_string(),
            description: "You are using a modern Java version with an old Forge version. Set the option to ignore certificate checks.".to_string(),
        },
        Issue::ChecksumMismatch => FormattedIssueInfo {
            title: "Checksum Mismatch".to_string(),
            description: "Outdated cached files detected. Please clear your cache.".to_string(),
        },
        Issue::NvidiaLinux => FormattedIssueInfo {
            title: "Nvidia Drivers on Linux".to_string(),
            description: "Check your Nvidia drivers on Linux to ensure they are correctly configured.".to_string(),
        },
        Issue::LinuxOpenal => FormattedIssueInfo {
            title: "Missing .alsoftrc".to_string(),
            description: "You are missing the .alsoftrc configuration file. Please create or restore it.".to_string(),
        },
        Issue::X11ConnectFailure => FormattedIssueInfo {
            title: "Failed to Connect to X11".to_string(),
            description: "This error often occurs with Flatpak. Ensure your Flatpak is configured correctly.".to_string(),
        },
        Issue::OldJavaMacOs => FormattedIssueInfo {
            title: "Old Java on MacOS".to_string(),
            description: "You are using an outdated version of Java on MacOS. Update your Java installation.".to_string(),
        },
        Issue::MissingXrandr => FormattedIssueInfo {
            title: "Missing xrandr".to_string(),
            description: "xrandr is missing and required for certain Minecraft versions using LWJGL 2.".to_string(),
        },
        Issue::InvalidFolderName => FormattedIssueInfo {
            title: "Invalid Folder Name".to_string(),
            description: "The folder name is invalid. Please rename your folder to a valid name.".to_string(),
        },
        Issue::InstanceDataCorrupted => FormattedIssueInfo {
            title: "Corrupted Instance Files".to_string(),
            description: "Your instance files are corrupted. Please restore from a backup.".to_string(),
        },
        Issue::InvalidProxy => FormattedIssueInfo {
            title: "Invalid Proxy Configuration".to_string(),
            description: "Please check your proxy settings, as they are currently invalid.".to_string(),
        },
        Issue::ShaderCompileError => FormattedIssueInfo {
            title: "Shader Compile Error".to_string(),
            description: "A GLSL shader could not be compiled. Check shader logs for more details.".to_string(),
        },
        Issue::ForgeSuspectedMod(suspected_mod_infos) => {
            let mods = suspected_mod_infos.iter()
                .map(|info| format!("{}", info.mod_name))
                .collect::<Vec<_>>()
                .join(", ");
            FormattedIssueInfo {
                title: "Suspected Mod".to_string(),
                description: format!("The following mods are suspected to be causing issues: {}", mods),
            }
        },
        Issue::EntrypointExecutionErrors(entrypoint_execution_errors) => {
            FormattedIssueInfo {
                title: "Entrypoint Execution Errors".to_string(),
                description: format!("There was an error during the entrypoint execution of mod {}", entrypoint_execution_errors.mod_name),
            }
        },
        Issue::CriticalInjectionFailure(critical_injection_failure) => {
            FormattedIssueInfo {
                title: "Critical Injection Failure".to_string(),
                description: format!("A critical injection failure has occurred for mod {}", critical_injection_failure.normalized_mod_name),
            }
        },
        Issue::ModsFoundInStacktrace(btree_set) => {
            let mods = btree_set.iter().cloned().collect::<Vec<_>>().join(", ");
            FormattedIssueInfo {
                title: "Mods Found in Stacktrace".to_string(),
                description: format!("The following mods were found in the stacktrace: {}", mods),
            }
        },
    }
}