use crate::issues::issue::Issue;

fn linux_openal(text: &str) -> Option<Issue> {
    text.contains("# C  [libopenal.so").then_some(Issue::LinuxOpenal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_linux_openal() {
        let text = r#"[15:35:50] [Render thread/WARN]: Missing sound for event: minecraft:item.goat_horn.play
[15:35:50] [Render thread/WARN]: Missing sound for event: minecraft:entity.goat.screaming.horn_break
[15:35:50] [Render thread/WARN]: Missing sound for event: aleradozemeorigins:power.taming_call
[15:35:50] [Render thread/WARN]: Missing sound for event: aleradozemeorigins:entity.war_wolf.howl
[ALSOFT] (EE) Failed to set real-time priority for thread: Operation not permitted (1)
[15:35:50] [Render thread/WARN]: Enhanced sound processing is auto disabled due to the presence of the mod "sound_physics_remastered"
[ALSOFT] (EE) Failed to set real-time priority for thread: Operation not permitted (1)
#
# A fatal error has been detected by the Java Runtime Environment:
#
#  SIGFPE (0x8) at pc=0x00007f416d49fb4d, pid=7620, tid=7945
#
# JRE version: OpenJDK Runtime Environment Microsoft-11369869 (17.0.15 6) (build 17.0.15 6-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-11369869 (17.0.15 6-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, linux-amd64)
# Problematic frame:
# C  [libopenal.so 0x9fb4d]
#
# Core dump will be written. Default location: Core dumps may be processed with "/usr/lib/systemd/systemd-coredump %P %u %g %s %t %c %h %d %F" (or dumping to /var/home/RIX/.local/share/PrismLauncher/instances/Aleradas-1.0.1/minecraft/core.7620)
#
# An error report file with more information is saved as:
# /var/home/RIX/.local/share/PrismLauncher/instances/Aleradas-1.0.1/minecraft/hs_err_pid7620.log
[thread 7621 also had an error]
#
# If you would like to submit a bug report, please visit:
#   https://aka.ms/minecraftjavacrashes
#
Process crashed with exitcode 6.
Log upload triggered at: 30 Nov 2025 15:36:31  0200"#;
        let issue = linux_openal(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::LinuxOpenal);
    }
}