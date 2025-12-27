use crate::header::{extract::{JavaVersionInfo, ModInfo}, index::IndexedLogHeader};

#[allow(dead_code)]
/// All extractable info from header.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogHeaderInfo {
    online_mode: Option<bool>,
    mc_folder_location: Option<String>,
    java_path: Option<String>,
    java_version: Option<JavaVersionInfo>,
    hardware_info: Option<String>,
    opengl_version: Option<String>,
    kernel_driver: Option<String>,
    main_class: Option<String>,
    native_path: Option<String>,
    instance_name: Option<String>,
    traits: Option<Vec<String>>,
    libraries: Option<Vec<String>>,
    native_libraries: Option<Vec<String>>,
    mods: Option<Vec<ModInfo>>,
    params: Option<String>,
    window_size: Option<(usize, usize)>,
    java_arguments: Option<Vec<String>>,
    mc_process_id: Option<usize>,
    jvm_info: Option<String>,
    ipv4_preferred: Option<bool>,
    current_time: Option<String>,
    created_tmp_dir: Option<String>,
}

impl LogHeaderInfo {
    pub fn from_indexed_header(header: &IndexedLogHeader) -> Self {
        Self {
            online_mode: header.get_online_mode(),
            mc_folder_location: header.get_mc_folder_location().map(|s| s.to_owned()),
            java_path: header.get_java_path().map(|s| s.to_owned()),
            java_version: header.get_java_version(),
            hardware_info: header.get_hardware_info(),
            opengl_version: header.get_opengl_version().map(|s| s.to_owned()),
            kernel_driver: header.get_kernel_driver().map(|s| s.to_owned()),
            main_class: header.get_main_class().map(|s| s.to_owned()),
            native_path: header.get_native_path().map(|s| s.to_owned()),
            instance_name: header.get_instance_name().map(|s| s.to_owned()),
            traits: header.get_traits(),
            libraries: header.get_libraries(),
            native_libraries: header.get_native_libraries(),
            mods: header.get_mods(),
            params: header.get_params().map(|s| s.to_owned()),
            window_size: header.get_window_size(),
            java_arguments: header.get_java_arguments(),
            mc_process_id: header.get_mc_process_id(),
            jvm_info: header.get_jvm_info().map(|s| s.to_owned()),
            ipv4_preferred: header.get_ipv4_preferred(),
            current_time: header.get_current_time().map(|s| s.to_owned()),
            created_tmp_dir: header.get_created_tmp_dir().map(|s| s.to_owned()),
        }
    }
}