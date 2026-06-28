pub struct IndexedLogHeader<'a> {
    pub index: LogHeaderIndex,
    pub text: &'a str,
}

#[allow(dead_code)]
impl IndexedLogHeader<'_> {
    pub fn index_header<'a>(header: &'a str) -> IndexedLogHeader<'a> {
        IndexedLogHeader {
            index: LogHeaderIndex::index_header(header),
            text: header,
        }
    }

    pub fn from_index<'a>(index: LogHeaderIndex, header: &'a str) -> IndexedLogHeader<'a> {
        IndexedLogHeader {
            index,
            text: header,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct LogHeaderIndex {
    pub online_mode: Option<usize>, // L4 e.g. "Launched instance in online mode" + DNS or "Launched instance in offline mode"
    pub mc_folder_location: Option<usize>, // e.g. "Minecraft folder is:"
    pub java_path: Option<usize>,   // e.g. "Java path is:"
    pub java_version: Option<usize>, // e.g. "Java is version"
    pub pre_launch_command: Option<usize>, // e.g. "Running Pre-Launch command:"
    pub kernel_driver: Option<usize>, // e.g. "Kernel driver in use:"
    pub opengl_version: Option<usize>, // e.g. "OpenGL version string:"
    pub main_class: Option<usize>,  // e.g. "Main Class:"
    pub native_path: Option<usize>, // e.g. "Native path:"
    pub traits: Option<usize>,      // e.g. "Traits:"
    pub libraries: Option<usize>,   // e.g. "Libraries:"
    pub native_libraries: Option<usize>, // e.g. "Native libraries:"
    pub mods: Option<usize>,        // e.g. "Mods:"
    pub params: Option<usize>,      // e.g. "Params:"
    pub window_size: Option<usize>, // e.g. "Window size:"
    pub launcher: Option<usize>,    // e.g. "Launcher:"
    pub java_arguments: Option<usize>, // e.g. "Java Arguments:"
    pub mc_process_id: Option<usize>, // e.g. "Minecraft process ID:"
    pub jvm_info: Option<usize>,    // e.g. "JVM info:"
    pub current_time: Option<usize>, // e.g. "Current Time:"
    pub created_tmp_dir: Option<usize>, // e.g. "Created Temporary Directory:"
    pub building_processors: Option<usize>, // e.g. "Building Processors"
}

impl LogHeaderIndex {
    pub fn index_header(log_header: &str) -> Self {
        index(log_header)
    }
}

fn find_section_start(log_header: &str, start_index: usize, prefixes: &[&str]) -> Option<usize> {
    let s = log_header.get(start_index..)?;
    for prefix in prefixes {
        if let Some(idx) = s.find(prefix) {
            return Some(idx + start_index);
        }
    }
    return None;
}

// Very verbose, but can't think of a better way right now that also avoids scanning the entire header over and over
fn index(log_header: &str) -> LogHeaderIndex {
    let mut index = 0_usize; // Keeps track of the last found position to optimize searching
    let online_mode = log_header.find("Launched instance in");
    if let Some(p) = online_mode {
        index = p;
    }
    let mc_folder_location = find_section_start(log_header, index, &["Minecraft folder is:"]);
    if let Some(p) = mc_folder_location {
        index = p;
    }
    let java_path = find_section_start(log_header, index, &["Java path is:"]);
    if let Some(p) = java_path {
        index = p;
    }
    let java_version = find_section_start(log_header, index, &["Java is version"]);
    if let Some(p) = java_version {
        index = p;
    }
    let pre_launch_command = find_section_start(log_header, index, &["Running Pre-Launch command:"]);
    if let Some(p) = pre_launch_command {
        index = p;
    }
    let kernel_driver = find_section_start(log_header, index, &["Kernel driver in use:"]);
    if let Some(p) = kernel_driver {
        index = p;
    }
    let opengl_version = find_section_start(log_header, index, &["OpenGL version string:"]);
    if let Some(p) = opengl_version {
        index = p;
    }
    let main_class = find_section_start(log_header, index, &["Main Class:", "Main class:"]);
    if let Some(p) = main_class {
        index = p;
    }
    let native_path = find_section_start(log_header, index, &["Native path:", "Natives path:"]);
    if let Some(p) = native_path {
        index = p;
    }
    let traits = find_section_start(log_header, 0, &["Traits:"]);
    if let Some(p) = traits {
        index = p;
    }
    let libraries = find_section_start(log_header, 0, &["Libraries:"]);
    if let Some(p) = libraries {
        index = p;
    }
    let native_libraries = find_section_start(log_header, index, &["Native libraries:"]);
    if let Some(p) = native_libraries {
        index = p;
    }
    let mods = find_section_start(log_header, 0, &["Mods:"]);
    if let Some(p) = mods {
        index = p;
    }
    let params = find_section_start(log_header, 0, &["Params:", "Minecraft arguments:"]);
    if let Some(p) = params {
        index = p;
    }
    let window_size = find_section_start(log_header, index, &["Window size:"]);
    if let Some(p) = window_size {
        index = p;
    }
    let launcher = find_section_start(log_header, 0, &["Launcher:"]);
    if let Some(p) = launcher {
        index = p;
    }
    let java_arguments = find_section_start(log_header, index, &["Java Arguments:", "Java arguments:"]);
    if let Some(p) = java_arguments {
        index = p;
    }
    let mc_process_id = find_section_start(log_header, index, &["Minecraft process ID:"]);
    if let Some(p) = mc_process_id {
        index = p;
    }
    let jvm_info = find_section_start(log_header, index, &["JVM info:"]);
    if let Some(p) = jvm_info {
        index = p;
    }
    let current_time = find_section_start(log_header, index, &["Current Time:"]);
    if let Some(p) = current_time {
        index = p;
    }
    let created_tmp_dir = find_section_start(log_header, index, &["Created Temporary Directory:"]);
    if let Some(p) = created_tmp_dir {
        index = p;
    }
    let building_processors = find_section_start(log_header, index, &["Building Processors"]); 
  
    LogHeaderIndex {
        online_mode,
        mc_folder_location,
        java_path,
        java_version,
        pre_launch_command,
        kernel_driver,
        opengl_version,
        main_class,
        native_path,
        traits,
        libraries,
        native_libraries,
        mods,
        params,
        window_size,
        launcher,
        java_arguments,
        mc_process_id,
        jvm_info,
        current_time,
        created_tmp_dir,
        building_processors,
    }
}

#[cfg(test)]
mod tests {
    use super::index;
    #[test]
    fn index_header_1() {
        let log_header = include_str!("test_data/header_1.log");
        let locations = index(&log_header);
        assert_eq!(locations.online_mode, Some(41));
        assert_eq!(locations.mc_folder_location, Some(835));
        assert_eq!(locations.java_path, Some(930));
        assert_eq!(locations.java_version, Some(1112));
        assert_eq!(locations.pre_launch_command, None);
        assert_eq!(locations.kernel_driver, None);
        assert_eq!(locations.opengl_version, None);
        assert_eq!(locations.main_class, Some(1185));
        assert_eq!(locations.native_path, Some(1251));
        assert_eq!(locations.traits, Some(1337));
        assert_eq!(locations.libraries, Some(1473));
        assert_eq!(locations.native_libraries, Some(13296));
        assert_eq!(locations.mods, Some(13315));
        assert_eq!(locations.params, Some(21507));
        assert_eq!(locations.window_size, Some(21913));
        assert_eq!(locations.launcher, Some(21937));
        assert_eq!(locations.java_arguments, Some(21957));
        assert_eq!(locations.mc_process_id, Some(22108));
        assert_eq!(locations.jvm_info, Some(22137));
        assert_eq!(locations.current_time, Some(22212));
        assert_eq!(locations.created_tmp_dir, Some(22246));
        assert_eq!(locations.building_processors, Some(22379));
    }

    #[test]
    fn index_header_4() {
        let log_header = include_str!("test_data/header_4.log");
        let locations = index(&log_header);
        assert_eq!(locations.online_mode, Some(43));
        assert_eq!(locations.mc_folder_location, Some(817));
        assert_eq!(locations.java_path, Some(916));
        assert_eq!(locations.java_version, Some(1018));
        assert_eq!(locations.pre_launch_command, None);
        assert_eq!(locations.kernel_driver, None);
        assert_eq!(locations.opengl_version, None);
        assert_eq!(locations.main_class, Some(1439));
        assert_eq!(locations.native_path, Some(27053));
        assert_eq!(locations.traits, Some(13322));
        assert_eq!(locations.libraries, Some(13438));
        assert_eq!(locations.native_libraries, Some(27034));
        assert_eq!(locations.mods, Some(1503));
        assert_eq!(locations.params, Some(27143));
        assert_eq!(locations.window_size, Some(27563));
        assert_eq!(locations.launcher, Some(1420));
        assert_eq!(locations.java_arguments, Some(27587));
        assert_eq!(locations.mc_process_id, Some(27733));
        assert_eq!(locations.jvm_info, Some(27763));
        assert_eq!(locations.current_time, Some(27837));
        assert_eq!(locations.created_tmp_dir, Some(27871));
        assert_eq!(locations.building_processors, Some(28008));
    }
}