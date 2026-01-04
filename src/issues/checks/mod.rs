use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub mod flatpak_nvidia;
pub mod fabric_internal;
pub mod lexforge_zlibng;
pub mod forge_java;
pub mod intel_hd;
pub mod java_option;
pub mod lwjgl_2_java_9;
pub mod macos_ns;
pub mod oom;
pub mod optifine;
pub mod outdated_launcher;
pub mod native_transport;
pub mod wrong_java;
pub mod forge_missing_dependencies;
pub mod new_java_old_forge;
pub mod locked_jar;
pub mod missing_libraries;
pub mod missing_indium;
pub mod no_disk_space;
pub mod java_32_bit;
pub mod intermediary_mappings;
pub mod checksum_mismatch;
pub mod nvidia_linux;
pub mod linux_openal;
pub mod x11_connect_failure;
pub mod old_java_macos;
pub mod missing_xrandr;
pub mod invalid_folder_name;
pub mod corrupted_instance;
pub mod invalid_proxy;

#[allow(dead_code)]
const CHECKS_FULL_LOG: [for<'a> fn(&str) -> Option<super::issue::Issue>; 5] = [
    checksum_mismatch::checksum_mismatch,
    fabric_internal::fabric_internal,
    invalid_proxy::invalid_proxy,
    java_32_bit::java_32_bit,
    java_option::java_option,
];

#[allow(dead_code)]
const CHECKS_HEADER: [for<'a> fn(&str) -> Option<super::issue::Issue>; 4] = [
    corrupted_instance::corrupted_instance,
    invalid_folder_name::invalid_folder_name,
    lexforge_zlibng::lexforge_zlibng,
    locked_jar::locked_jar,
];

#[allow(dead_code)]
const CHECKS_ENTRIES: [for<'a> fn(&LogEntry) -> Option<Issue>; 5] = [
    flatpak_nvidia::flatpak_nvidia,
    forge_java::forge_java,
    forge_missing_dependencies::forge_missing_dependencies,
    // intel_hd::intel_hd,
    intermediary_mappings::intermediary_mappings,
    linux_openal::linux_openal,
];