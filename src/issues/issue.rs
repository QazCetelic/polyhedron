use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq)]
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
    Lwjgl2Java9,
    #[error("MacOS NSInternalInconsistencyException")]
    MacOSNSInternal,
    #[error("Out of Memory")]
    Oom,
    #[error("Optifine enabled")]
    Optifine,
    #[error("Outdated launcher")]
    OutdatedLauncher,
}