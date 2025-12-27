use std::collections::VecDeque;

use crate::header::index::IndexedLogHeader;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JavaVersionInfo {
    pub version: String,
    pub architecture: String,
    pub vendor: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModInfo {
    pub name: String,
    pub enabled: bool,
}

#[allow(dead_code)]
impl<'a> IndexedLogHeader<'a> {
    pub fn get_online_mode(&self) -> Option<bool> {
        let mode = self.header.get(self.index.online_mode?..)
            .and_then(|s| s.lines().next())?;
        match mode {
            "Launched instance in online mode" => Some(true),
            "Launched instance in offline mode" => Some(false),
            _ => None,
        }
    }

    pub fn get_mc_folder_location(&self) -> Option<&'a str> {
        self.header.get(self.index.mc_folder_location?..)?.lines().skip(1).next()
    }

    pub fn get_java_path(&self) -> Option<&'a str> {
        self.header.get(self.index.java_path?..)?.lines().skip(1).next()
    }

    pub fn get_java_version(&self) -> Option<JavaVersionInfo> {
        // e.g. "1.8.0_202, using 64 (amd64) architecture, from Oracle Corporation."
        let java_version_line = self.header.get(self.index.java_version?..)?.lines().next()?.strip_prefix("Java is version ")?;
        let (version, rest) = java_version_line.split_once(", using ")?;
        let (architecture, vendor_part) = rest.split_once(" architecture, from ")?;
        let vendor = vendor_part.trim_end_matches('.');
        Some(JavaVersionInfo {
            version: version.to_string(),
            architecture: architecture.to_string(),
            vendor: vendor.to_string(),
        })
    }

    pub fn get_hardware_info(&self) -> Option<String> {
        let java_version_index = self.index.java_version?;
        let kernel_driver_index = self.index.kernel_driver?;
        let str = self.header.get(java_version_index..kernel_driver_index)?;
        
        let mut lines: VecDeque<&str> = str.lines().collect();
        lines.pop_front()?; // // Removes java version line
        lines.pop_back()?; // Remove kernel driver line

        while lines.front()?.is_empty() {
            lines.pop_front();
        }
        while lines.back()?.is_empty() {
            lines.pop_back();
        }

        let lines_vec: Vec<&str> = lines.into(); 
        Some(lines_vec.join("\n"))
    }

    pub fn get_opengl_version(&self) -> Option<&'a str> {
        // e.g. "OpenGL version string: 4.6 (Compatibility Profile) Mesa 25.2.7"
        Some(self.header.get(self.index.opengl_version?..)?.lines().next()?.strip_prefix("OpenGL version string: ")?)
    }

    /// "Linux Kernel Graphics Driver, e.g. nvidia, amdgpu, intel"
    pub fn get_kernel_driver(&self) -> Option<&'a str> {
        // e.g. "Kernel driver in use: nvidia", "Kernel driver in use: amdgpu"
        Some(self.header.get(self.index.kernel_driver?..)?.lines().next()?.strip_prefix("Kernel driver in use: ")?)
    }

    pub fn get_main_class(&self) -> Option<&'a str> {
        Some(self.header.get(self.index.main_class?..)?.lines().skip(1).next()?.trim_ascii_start())
    }

    pub fn get_native_path(&self) -> Option<&'a str> {
        Some(self.header.get(self.index.native_path?..)?.lines().skip(1).next()?.trim_ascii_start())
    }

    pub fn get_instance_name(&self) -> Option<&'a str> {
        let native_path = self.get_native_path()?.strip_suffix("/natives")?;
        for (r_idx, byte) in native_path.as_bytes().iter().rev().enumerate() {
            if *byte == b'/' || *byte == b'\\' {
                let l_idx = native_path.len() - r_idx;
                return Some(&native_path.get(l_idx..)?);
            }
        }
        None
    }

    pub fn get_traits(&self) -> Option<Vec<String>> {
        let mut lines = self.header.get(self.index.traits?..)?.lines().skip(1);
        let mut traits = Vec::new();
        while let Some(line) = lines.next() && line.starts_with("traits ") {
            let trait_name = line.strip_prefix("traits ")?.to_string();
            traits.push(trait_name);
        }
        Some(traits)
    }

    fn get_libraries_section(&self, start_index: usize) -> Option<Vec<String>> {
        let mut lines = self.header.get(start_index..)?.lines().skip(1);
        let mut libraries = Vec::new();
        while let Some(line) = lines.next() && line.starts_with("  ") { // Libraries are indented by two spaces
            let library_name = line.strip_prefix("  ")?.to_string();
            libraries.push(library_name);
        }
        Some(libraries)
    }

    pub fn get_libraries(&self) -> Option<Vec<String>> {
        Self::get_libraries_section(&self, self.index.libraries?)
    }

    pub fn get_native_libraries(&self) -> Option<Vec<String>> {
        Self::get_libraries_section(&self, self.index.native_libraries?)
    }

    pub fn get_mods(&self) -> Option<Vec<ModInfo>> {
        let mut lines = self.header.get(self.index.mods?..)?.lines().skip(1);
        let mut mods: Vec<ModInfo> = Vec::new();
        while let Some(line) = lines.next() && line.starts_with("  [") {
            // line e.g. "  [✔] Patchouli-1.20.1-84.1-FORGE"
            let mod_line = line.strip_prefix("  [")?;
            if mod_line.starts_with("🖿]") {
                continue; // Skip folder entries
            }
            let (mod_enabled_char, name) = mod_line.split_once("] ")?;
            let enabled = match mod_enabled_char {
                "✔" => true,
                "✘" => false,
                _ => return None,
            };
            mods.push(ModInfo {
                name: name.trim_end_matches(" (disabled)").to_string(),
                enabled,
            });
        }
        Some(mods)
    }

    pub fn get_params(&self) -> Option<&'a str> {
        Some(self.header.get(self.index.params?..)?.lines().skip(1).next()?.trim_ascii_start())
    }

    pub fn get_window_size(&self) -> Option<(usize, usize)> {
        // e.g. "Window size: 854 x 480"
        let size_line = self.header.get(self.index.window_size?..)?.lines().next()?.strip_prefix("Window size: ")?;
        let (width, height) = size_line.split_once(" x ")?;
        Some((width.parse().ok()?, height.parse().ok()?))
    }

    pub fn get_java_arguments(&self) -> Option<Vec<String>> {
        // e.g. "[-Xms512m, -Xmx8096m, -Duser.language=en]"
        let args_line = self.header.get(self.index.java_arguments?..)?.lines().skip(1).next()?.strip_prefix("[")?.strip_suffix("]")?;
        let mut args: Vec<String> = Vec::new();
        for arg in args_line.split(", ") {
            args.push(arg.to_string());
        }
        Some(args)
    }

    pub fn get_mc_process_id(&self) -> Option<usize> {
        let pid_line = self.header.get(self.index.mc_process_id?..)?.lines().next()?.strip_prefix("Minecraft process ID: ")?;
        Some(pid_line.parse().ok()?)
    }

    pub fn get_jvm_info(&self) -> Option<&'a str> {
        // e.g. "JVM info: Microsoft - 17.0.15 - 17.0.15 6-LTS"
        Some(self.header.get(self.index.jvm_info?..)?.lines().next()?.strip_prefix("JVM info: ")?.trim_ascii_start())
    }

    pub fn get_ipv4_preferred(&self) -> Option<bool> {
        let ipv4_line = self.header.get(self.index.jvm_info?..)?.lines().skip(1).next()?;
        match ipv4_line.strip_prefix("java.net.preferIPv4Stack=")? {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    pub fn get_current_time(&self) -> Option<&'a str> { // TODO consider parsing later
        // e.g. "Current Time: 17/10/2025 17:57:39"
        Some(self.header.get(self.index.current_time?..)?.lines().next()?.strip_prefix("Current Time: ")?.trim_ascii_start())
    }

    pub fn get_created_tmp_dir(&self) -> Option<&'a str> {
        // e.g. "Created Temporary Directory: /tmp/forge_installer2996225851521709060"
        Some(self.header.get(self.index.created_tmp_dir?..)?.lines().next()?.strip_prefix("Created Temporary Directory: ")?.trim_ascii_start())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::header::index::{IndexedLogHeader};

    #[test]
    fn extract_from_header_1() {
        let string = include_str!("test_data/header_1.log");
        let index = IndexedLogHeader::index_header(string);
        let online_mode = index.get_online_mode().expect("Failed to get online mode");
        assert_eq!(online_mode, true);
        let mc_folder_location_str = index.get_mc_folder_location().expect("Failed to get MC folder location");
        assert_eq!(mc_folder_location_str, "C:/Users/********/AppData/Roaming/PrismLauncher/instances/guh/minecraft");
        let java_path_str = index.get_java_path().expect("Failed to get Java path");
        assert_eq!(java_path_str, "C:/Users/********/AppData/Local/Packages/Microsoft.4297127D64EC6_8wekyb3d8bbwe/LocalCache/Local/runtime/java-runtime-beta/windows-x64/java-runtime-beta/bin/javaw.exe");
        let java_version_info = index.get_java_version().expect("Failed to get Java version");
        assert_eq!(java_version_info.version, "17.0.1");
        assert_eq!(java_version_info.architecture, "64 (amd64)");
        assert_eq!(java_version_info.vendor, "Microsoft");
        let kernel_driver_in_use = index.get_kernel_driver();
        assert_eq!(kernel_driver_in_use, None);
        let main_class_str = index.get_main_class().expect("Failed to get main class");
        assert_eq!(main_class_str, "io.github.zekerzhayard.forgewrapper.installer.Main");
        let native_path_str = index.get_native_path().expect("Failed to get native path");
        assert_eq!(native_path_str, "C:/Users/********/AppData/Roaming/PrismLauncher/instances/guh/natives");
        let instance_name = index.get_instance_name().expect("Failed to get instance name");
        assert_eq!(instance_name, "guh");
        let traits = index.get_traits().expect("Failed to get traits");
        assert_eq!(traits, vec!["feature:is_quick_play_multiplayer", "XR:Initial", "FirstThreadOnMacOS", "feature:is_quick_play_singleplayer"]);
        let libraries = index.get_libraries().expect("Failed to get libraries");
        assert!(libraries.contains(&"C:/Users/********/AppData/Roaming/PrismLauncher/libraries/org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1.jar".to_string()));
        let mods = index.get_mods().expect("Failed to get mods");
        assert!(mods.iter().any(|m| m.name == "Patchouli-1.20.1-84.1-FORGE" && m.enabled));
        let params_str = index.get_params().expect("Failed to get params");
        assert!(params_str.starts_with("--username  --version 1.20.1 --gameDir"));
        let (width, height) = index.get_window_size().expect("Failed to get window size");
        assert_eq!(width, 854);
        assert_eq!(height, 480);
        let java_arguments = index.get_java_arguments().expect("Failed to get Java arguments");
        assert_eq!(java_arguments, vec!["-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump", "-Xms512m", "-Xmx25728m", "-Duser.language=en"]);
        let mc_process_id = index.get_mc_process_id().expect("Failed to get MC process ID");
        assert_eq!(mc_process_id, 4860);
        let jvm_info_str = index.get_jvm_info().expect("Failed to get JVM info");
        assert_eq!(jvm_info_str, "Microsoft - 17.0.1 - 17.0.1 12-LTS");
        let ipv4_preferred = index.get_ipv4_preferred().expect("Failed to get IPv4 stack preference");
        assert_eq!(ipv4_preferred, true);
        let current_time_str = index.get_current_time().expect("Failed to get current time");
        assert_eq!(current_time_str, "18/09/2025 18:12:22");
        let created_tmp_dir_str = index.get_created_tmp_dir().expect("Failed to get created temp dir");
        assert_eq!(created_tmp_dir_str, "C:\\Users\\********\\AppData\\Local\\Temp\\forge_installer8903858996068324158");
    }

    #[test]
    fn extract_from_header_2() {
        let string = include_str!("test_data/header_2.log");
        let index = IndexedLogHeader::index_header(string);
        let online_mode = index.get_online_mode().expect("Failed to get online mode");
        assert_eq!(online_mode, true);
        let mc_folder_location_str = index.get_mc_folder_location().expect("Failed to get MC folder location");
        assert_eq!(mc_folder_location_str, "/var/home/RIX/.local/share/PrismLauncher/instances/Aleradas-openBeta-2.0.0/minecraft");
        let java_path_str = index.get_java_path().expect("Failed to get Java path");
        assert_eq!(java_path_str, "/var/home/RIX/.local/share/PrismLauncher/java/java-runtime-gamma/bin/java");
        let java_version_info = index.get_java_version().expect("Failed to get Java version");
        assert_eq!(java_version_info.version, "17.0.15");
        assert_eq!(java_version_info.architecture, "64 (amd64)");
        assert_eq!(java_version_info.vendor, "Microsoft");
        let kernel_driver_in_use = index.get_kernel_driver().expect("Failed to get kernel driver");
        assert_eq!(kernel_driver_in_use, "amdgpu");
        let hardware_info = index.get_hardware_info().expect("Failed to get hardware info");
        assert_eq!(hardware_info, "AMD Ryzen 5 7600X 6-Core Processor\nAdvanced Micro Devices, Inc. [AMD/ATI] Navi 44 [Radeon RX 9060 XT] (rev c0)\n\nSubsystem: XFX Limited Device 8601");
        let opengl_version = index.get_opengl_version().expect("Failed to get OpenGL version");
        assert_eq!(opengl_version, "4.6 (Compatibility Profile) Mesa 25.2.7");
        let main_class_str = index.get_main_class().expect("Failed to get main class");
        assert_eq!(main_class_str, "net.fabricmc.loader.impl.launch.knot.KnotClient");
        let native_path_str = index.get_native_path().expect("Failed to get native path");
        assert_eq!(native_path_str, "/var/home/RIX/.local/share/PrismLauncher/instances/Aleradas-openBeta-2.0.0/natives");
        let instance_name = index.get_instance_name().expect("Failed to get instance name");
        assert_eq!(instance_name, "Aleradas-openBeta-2.0.0");
        let traits = index.get_traits().expect("Failed to get traits");
        assert_eq!(traits, vec!["feature:is_quick_play_multiplayer", "XR:Initial", "feature:is_quick_play_singleplayer", "FirstThreadOnMacOS"]);
        let libraries = index.get_libraries().expect("Failed to get libraries");
        assert!(libraries.contains(&"/var/home/RIX/.local/share/PrismLauncher/libraries/org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1.jar".to_string()));
        let mods = index.get_mods().expect("Failed to get mods");
        assert!(mods.iter().any(|m| m.name == "Botania-1.20.1-448-FABRIC" && m.enabled));
        let params_str = index.get_params().expect("Failed to get params");
        assert!(params_str.starts_with("--username  --version 1.20.1 --gameDir"));
        let (width, height) = index.get_window_size().expect("Failed to get window size");
        assert_eq!(width, 854);
        assert_eq!(height, 480);
        let java_arguments = index.get_java_arguments().expect("Failed to get Java arguments");
        assert_eq!(java_arguments, vec!["-Xms512m", "-Xmx10024m", "-Duser.language=en"]);
        let mc_process_id = index.get_mc_process_id().expect("Failed to get MC process ID");
        assert_eq!(mc_process_id, 6506);
        let jvm_info_str = index.get_jvm_info();
        assert_eq!(jvm_info_str, None);
        let ipv4_preferred = index.get_ipv4_preferred();
        assert_eq!(ipv4_preferred, None);
        let current_time_str = index.get_current_time();
        assert_eq!(current_time_str, None);
        let created_tmp_dir_str = index.get_created_tmp_dir();
        assert_eq!(created_tmp_dir_str, None);
    }
}