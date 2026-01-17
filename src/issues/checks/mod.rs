use crate::{entries::entry::LogEntry, header::index::IndexedLogHeader, issues::{checks::intel_hd::intel_hd_entry, issue::Issue}, parse::crash_report::CrashReport};

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
pub mod shader_compile_error;
pub mod suspected_mod;
pub mod entrypoint_execution_errors;

#[allow(dead_code)]
pub const CHECKS_TEXT: [for<'a> fn(&IndexedLogHeader<'a>) -> Box<dyn Fn(&str) -> Option<Issue>>; 3] = [
    |_header| { 
        Box::new(|text| fabric_internal::fabric_internal(text)) 
    },
    |_header| { 
        Box::new(|text| java_32_bit::java_32_bit(text)) 
    },
    |_header| { 
        Box::new(|text| x11_connect_failure::x11_connect_failure(text)) 
    }
];

#[allow(dead_code)]
pub const CHECKS_CRASH_REPORT: [fn(&CrashReport) -> Option<Issue>; 2] = [
    suspected_mod::check_suspected_mod_crash_report,
    entrypoint_execution_errors::entrypoint_execution_errors,
];

#[allow(dead_code)]
pub const CHECKS_HEADER: [for<'a> fn(&IndexedLogHeader<'a>) -> Option<Issue>; 9] = [
    optifine::optifine_header,
    corrupted_instance::corrupted_instance,
    invalid_folder_name::invalid_folder_name_header,
    lexforge_zlibng::lexforge_zlibng_header,
    wrong_java::wrong_java_header,
    locked_jar::locked_jar_header,
    java_option::java_option,
    missing_libraries::missing_libraries_header,
    outdated_launcher::outdated_launcher_header,
];

#[allow(dead_code)]
pub const CHECKS_ENTRIES: [for<'a, 'b> fn(&IndexedLogHeader<'a>) -> Box<dyn Fn(&LogEntry) -> Option<Issue>>; 20] = [
    |header| {
        let java_version = header.get_java_version();
        Box::new(move |entry| intel_hd_entry(entry, java_version.as_ref()))
    },
    |_header| { 
        Box::new(|entry| invalid_proxy::invalid_proxy_entry(entry)) 
    },
    |_header| { 
        Box::new(|entry| checksum_mismatch::checksum_mismatch_entry(entry)) 
    },
    |_header| { 
        Box::new(|entry| flatpak_nvidia::flatpak_nvidia(entry)) 
    },
    |_header| { 
        Box::new(|entry| forge_java::forge_java(entry)) 
    },
    |_header| { 
        Box::new(|entry| forge_missing_dependencies::forge_missing_dependencies(entry)) 
    },
    |_header| { 
        Box::new(|entry| intermediary_mappings::intermediary_mappings(entry)) 
    },
    |_header| { 
        Box::new(|entry| linux_openal::linux_openal(entry)) 
    },
    |_header| { 
        Box::new(|entry| macos_ns::macos_ns(entry)) 
    },
    |_header| { 
        Box::new(|entry| missing_indium::missing_indium(entry)) 
    },
    |_header| { 
        Box::new(|entry| missing_xrandr::missing_xrandr(entry)) 
    },
    |_header| { 
        Box::new(|entry| native_transport::pre_1_12_native_transport_java_9(entry)) 
    },
    |_header| { 
        Box::new(|entry| no_disk_space::no_disk_space(entry)) 
    },
    |_header| { 
        Box::new(|entry| nvidia_linux::nvidia_linux(entry)) 
    },
    |_header| { 
        Box::new(|entry| old_java_macos::old_java_macos(entry)) 
    },
    |_header| { 
        Box::new(|entry| oom::oom(entry)) 
    },
    |_header| { 
        Box::new(|entry| lwjgl_2_java_9::lwjgl_2_java_9_entry(entry)) 
    },
    |_header| { 
        Box::new(|entry| new_java_old_forge::new_java_old_forge_legacy_java_fixer(entry)) 
    },
    |_header| { 
        Box::new(|entry| new_java_old_forge::new_java_old_forge_ignore_certificates(entry)) 
    },
    |_header| { 
        Box::new(|entry| shader_compile_error::shader_compile_error(entry)) 
    },
];