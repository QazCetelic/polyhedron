use crate::header::{extract::{JavaVersionInfo, LibraryInfo, ModInfo}, index::IndexedLogHeader};

#[allow(dead_code)]
/// All extractable info from header.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dioxius", derive(Clone, PartialEq))]
pub struct LogHeaderInfo {
    pub online_mode: Option<bool>,
    pub mc_folder_location: Option<String>,
    pub java_path: Option<String>,
    pub java_version: Option<JavaVersionInfo>,
    pub hardware_info: Option<String>,
    pub opengl_version: Option<String>,
    pub kernel_driver: Option<String>,
    pub main_class: Option<String>,
    pub native_path: Option<String>,
    pub instance_name: Option<String>,
    pub traits: Option<Vec<String>>,
    pub libraries: Option<Vec<String>>,
    pub native_libraries: Option<Vec<String>>,
    pub mods: Option<Vec<ModInfo>>,
    pub params: Option<String>,
    pub window_size: Option<(usize, usize)>,
    pub java_arguments: Option<Vec<String>>,
    pub mc_process_id: Option<usize>,
    pub jvm_info: Option<String>,
    pub ipv4_preferred: Option<bool>,
    pub current_time: Option<String>,
    pub created_tmp_dir: Option<String>,
}

impl LogHeaderInfo {
    pub fn from_indexed_header(header: &IndexedLogHeader) -> Self {
        fn take_lib_names(libs: Option<Vec<LibraryInfo>>) -> Option<Vec<String>> {
            let mut names = Vec::new();
            for lib in libs? {
                names.push(lib.name);
            }
            Some(names)
        }
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
            libraries: take_lib_names(header.get_libraries()),
            native_libraries: take_lib_names(header.get_native_libraries()),
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