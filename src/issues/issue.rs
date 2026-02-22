use std::collections::BTreeSet;
use thiserror::Error;
use crate::{issues::checks::{critical_injection_failure::CriticalInjectionFailure, entrypoint_execution_errors::EntrypointExecutionErrors, suspected_mod::SuspectedModInfo}, parse::jre_fatal::JreFatalError};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum Issue {
    #[error("Flatpak Nvidia driver is outdated")]
    OutdatedFlatpakNvidiaDriver,
    #[error("The mod you are using is using fabric internals")]
    FabricInternalAccess,
    #[error("Forge on zlib-ng")]
    LexforgeZlibng,
    #[error("Forge Java Bug")]
    ForgeJava,
    #[error("Intel HD Windows 10")]
    IntelHd,
    /// Argument is the Java arg to remove
    #[error("Wrong Java Option")]
    JavaOption(String),
    #[error("Crash with pre-1.13 and Java 9+")]
    Lwjgl2JavaAbove8,
    #[error("MacOS NSInternalInconsistencyException")]
    MacOSNSInternal,
    #[error("Out of Memory")]
    Oom,
    #[error("Optifine enabled")]
    Optifine,
    #[error("Outdated launcher")]
    OutdatedLauncher,
    #[error("These versions of Minecraft use an outdated version of Netty which does not properly support Java 9")]
    NettyJavaAbove8,
    /// Argument is the recommended java version (major)
    #[error("Wrong Java Version")]
    WrongJava(Option<u32>),
    #[error("Missing forge mod dependencies")]
    ForgeMissingDependencies,
    #[error("Modern Java version with an old Forge version, install LegacyJavaFixer")]
    NewJavaOldForgeLegacyJavaFixer,
    /// Argument is the files that are in use
    #[error("Locked Jars")]
    LockedJars(Vec<String>),
    #[error("Missing Libraries")]
    MissingLibraries(Vec<String>),
    #[error("Missing Indium")]
    MissingIndium,
    #[error("Out of disk space")]
    NoDiskSpace,
    #[error("32 bit Java crash")]
    Java32BitMemoryLimit,
    #[error("Wrong Intermediary Mappings version")]
    WrongIntermediaryMappingsVersion,
    #[error("Modern Java version with an old Forge version, disable certificate check")]
    NewJavaOldForgeIgnoreCerts,
    #[error("Outdated cached files")]
    ChecksumMismatch,
    #[error("Nvidia drivers on Linux")]
    NvidiaLinux, // Note: Refraction recommends setting `__GL_THREADED_OPTIMIZATIONS` to `0`, but the most common solution on Discord is disabling zink 
    #[error("Missing .alsoftrc")]
    LinuxOpenal,
    #[error("Failed to connect to X11")] // Note: Often a problem with Flatpak
    X11ConnectFailure,
    #[error("Old Java on MacOS")]
    OldJavaMacOs,
    #[error("xrandr missing on Minecraft versions that use LWJGL 2")]
    MissingXrandr,
    #[error("Invalid folder name")]
    InvalidFolderName(char),
    #[error("Corrupted instance files")]
    InstanceDataCorrupted,
    #[error("Invalid proxy configuration")]
    InvalidProxy,
    #[error("Failed to compile GLSL shader")]
    ShaderCompileError,
    #[error("Suspected mod")]
    ForgeSuspectedMod(Vec<SuspectedModInfo>),
    #[error("Entrypoint Execution Errors")]
    EntrypointExecutionErrors(Box<EntrypointExecutionErrors>),
    #[error("Critical injection failure")]
    CriticalInjectionFailure(Box<CriticalInjectionFailure>),
    #[error("Found mods in stacktrace")]
    ModsFoundInStacktrace(BTreeSet<String>),
    #[error("Mixin apply for mod failed")]
    MixinApplyFailure(String),
    #[error("Java Runtime Environment had a fatal error")]
    FatalErrorJre(Box<JreFatalError>),
}