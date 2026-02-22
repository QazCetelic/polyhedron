#[derive(PartialEq, PartialOrd, Debug)]
#[allow(non_camel_case_types)]
enum JreFatalErrorParserState {
    S1_HEADER,
    S2_SPACING,
    S3_ERROR,
    // S4 spacing
    S5_CONTENTS,
}

#[derive(Debug)]
struct JreFatalErrorParser {
    state: JreFatalErrorParserState, // What the parser is expecting next
    error: String,
    contents: String,
}

impl JreFatalErrorParser {
    pub fn new() -> Self {
        Self {
            state: JreFatalErrorParserState::S1_HEADER,
            error: String::new(),
            contents: String::new(),
        }
    }

    fn parse_line(&mut self, l: &str) -> Result<Option<JreFatalError>, ()> {
        match self.state {
            JreFatalErrorParserState::S1_HEADER => {
                if l == "# A fatal error has been detected by the Java Runtime Environment:" {
                    self.state = JreFatalErrorParserState::S2_SPACING;
                }
                return Ok(None);
            }
            JreFatalErrorParserState::S2_SPACING => {
                if l == "#" {
                    self.state = JreFatalErrorParserState::S3_ERROR;
                    return Ok(None);
                }
                return Err(());
            }
            JreFatalErrorParserState::S3_ERROR => {
                if let Some(l) = l.strip_prefix("#  ") {
                    if !self.error.is_empty() { self.error.push('\n'); }
                    self.error.push_str(l);
                    return Ok(None);
                }
                if l == "#" {
                    self.state = JreFatalErrorParserState::S5_CONTENTS;
                    return Ok(None);
                }
                return Err(());
            }
            JreFatalErrorParserState::S5_CONTENTS => {
                if let Some(l) = l.strip_prefix('#') {
                    if l.is_empty() {
                        self.contents.push('\n');
                        return Ok(None);
                    }
                    if let Some(l) = l.strip_prefix(' ') {
                        // Bug report link at the end is used to determine when the end of the error is reached.
                        // Checking whether the line starts with # doesn't work reliably
                        if l == "If you would like to submit a bug report, please visit:" {
                            return Ok(Some(self.complete()));
                        } else {
                            if !self.contents.is_empty() { self.contents.push('\n'); }
                            self.contents.push_str(l);
                            return Ok(None);
                        }
                    }
                }
                if !self.contents.is_empty() { self.contents.push('\n'); }
                self.contents.push_str(l);
                return Ok(None);
            }
        }
    }

    fn complete(&mut self) -> JreFatalError {
        let error = std::mem::replace(&mut self.error, String::new());
        let contents = std::mem::replace(&mut self.contents, String::new());
        JreFatalError { error, contents }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JreFatalError {
    pub error: String,
    pub contents: String,
}

impl JreFatalError {
    pub fn parse(s: &str) -> Option<JreFatalError> {
        let mut parser = JreFatalErrorParser::new();
        for line in s.lines() {
            let Ok(res) = parser.parse_line(line) else {
                return None;
            };
            if let Some(s) = res {
                return Some(s);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r#"#@!@# Game crashed! Crash report saved to: #@!@# C:\Users\********\AppData\Roaming\PrismLauncher\instances\1.21.7\minecraft\crash-reports\crash-2025-11-24_18.34.09-client.txt
activate hook: GetRawInputBuffer#
# A fatal error has been detected by the Java Runtime Environment:
#
#  EXCEPTION_ACCESS_VIOLATION (0xc0000005) at pc=0x00007fffbd7ed900, pid=28548, tid=23080
#
# JRE version: OpenJDK Runtime Environment Microsoft-9388422 (21.0.3 9) (build 21.0.3 9-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-9388422 (21.0.3 9-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, windows-amd64)
# Problematic frame:
# C  [owe-client-x64.dll 0x3dd900]
#
# No core dump will be written. Minidumps are not enabled by default on client versions of Windows
#
# An error report file with more information is saved as:
# C:\Users\********\AppData\Roaming\PrismLauncher\instances\1.21.7\minecraft\hs_err_pid28548.log
#
# If you would like to submit a bug report, please visit:
#   https://aka.ms/minecraftjavacrashes
#
Proces zakończył z kodem -1.
Clipboard copy at: 24 Nov 2025 18:34:21  0100"#;
        let error_info = JreFatalError::parse(text).unwrap();
        dbg!(error_info);
    }

    #[test]
    fn example_2() {
        let text = r#"#
# A fatal error has been detected by the Java Runtime Environment:
#
#  EXCEPTION_ACCESS_VIOLATION (0xc0000005) at pc=0x00007ff80beae2ad, pid=3204, tid=14140
#
# JRE version: OpenJDK Runtime Environment Microsoft-9388422 (21.0.3 9) (build 21.0.3 9-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-9388422 (21.0.3 9-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, windows-amd64)
# Problematic frame:
# C  [nvoglv64.dll 0x151e2ad]
#
# No core dump will be written. Minidumps are not enabled by default on client versions of Windows
#
# An error report file with more information is saved as:
# C:\Users\********\AppData\Roaming\PrismLauncher\instances\OptiFabric\minecraft\hs_err_pid3204.log
[25.474s][warning][os] Loading hsdis library failed
#
# If you would like to submit a bug report, please visit:
#   https://aka.ms/minecraftjavacrashes
# The crash happened outside the Java Virtual Machine in native code.
# See problematic frame for where to report the bug.
#
Process crashed with exitcode -1073740791."#;
        let error_info = JreFatalError::parse(text).unwrap();
        dbg!(error_info);
    }

    #[test]
    fn example_3() {
        let text = r#"#
# A fatal error has been detected by the Java Runtime Environment:
#
#  EXCEPTION_ACCESS_VIOLATION (0xc0000005) at pc=0x00007ffff634cad1, pid=2008, tid=17752
#
# JRE version: OpenJDK Runtime Environment Temurin-21.0.9 10 (21.0.9 10) (build 21.0.9 10-LTS)
# Java VM: OpenJDK 64-Bit Server VM Temurin-21.0.9 10 (21.0.9 10-LTS, mixed mode, sharing, tiered, compressed oops, compressed class ptrs, g1 gc, windows-amd64)
# Problematic frame:
# V  [jvm.dll 0xcad1]
#
# No core dump will be written. Minidumps are not enabled by default on client versions of Windows
#
# An error report file with more information is saved as:
# E:\PrismLauncher-Windows-MinGW-w64-Portable-9.4\instances\BigChadGuys Plus (Non Cobblemon!)\minecraft\hs_err_pid2008.log
#
# If you would like to submit a bug report, please visit:
#   https://github.com/adoptium/adoptium-support/issues
#"#;
        let error_info = JreFatalError::parse(text).unwrap();
        dbg!(error_info);
    }

    #[test]
    fn example_4() {
        let text = r#"#
# A fatal error has been detected by the Java Runtime Environment:
#
#  SIGSEGV (0xb) at pc=0x0000000000000000, pid=3566, tid=3945
#
# JRE version: OpenJDK Runtime Environment Microsoft-11369869 (17.0.15 6) (build 17.0.15 6-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-11369869 (17.0.15 6-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, linux-amd64)
# Problematic frame:
# C  [spark-3a8acfe0b5d-libasyncProfiler.so.tmp 0x270e9][thread 3946 also had an error]
  VMThread::nativeThreadId(JNIEnv_*, _jobject*) 0x59
#
# Core dump will be written. Default location: Core dumps may be processed with "/usr/bin/python -m steamos_log_submitter.hooks.coredump %P %e %u %g %s %t %c %h %f %E" (or dumping to /home/********/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/Homestead/minecraft/core.3566)
#
# An error report file with more information is saved as:
# /home/********/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/Homestead/minecraft/hs_err_pid3566.log
#
# If you would like to submit a bug report, please visit:
#   https://aka.ms/minecraftjavacrashes
# The crash happened outside the Java Virtual Machine in native code.
# See problematic frame for where to report the bug.
#"#;
        let error_info = JreFatalError::parse(text).unwrap();
        dbg!(error_info);
    }

    #[test]
    fn example_5() {
        let text = r#"#
# A fatal error has been detected by the Java Runtime Environment:
#
#  SIGBUS (0xa) at pc=0x000000010d4c54e4, pid=57376, tid=237059
#
# JRE version: OpenJDK Runtime Environment Microsoft-8035246 (17.0.8 7) (build 17.0.8 7-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-8035246 (17.0.8 7-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, bsd-aarch64)
# Problematic frame:
# v  ~StubRoutines::SafeFetch32
#
# No core dump will be written. Core dumps have been disabled. To enable core dumping, try "ulimit -c unlimited" before starting Java again
#
# An error report file with more information is saved as:
# /Users/********/minecraft/Star Technology/minecraft/hs_err_pid57376.log
Compiled method (c2)   56042 16441   !   4       java.lang.invoke.MethodHandleImpl::makeGuardWithTest (159 bytes)
 total in heap  [0x000000010dcd7610,0x000000010dcd9ae8] = 9432
 relocation     [0x000000010dcd7768,0x000000010dcd79f0] = 648
 main code      [0x000000010dcd7a00,0x000000010dcd8a40] = 4160
 stub code      [0x000000010dcd8a40,0x000000010dcd8e68] = 1064
 oops           [0x000000010dcd8e68,0x000000010dcd8ec8] = 96
 metadata       [0x000000010dcd8ec8,0x000000010dcd9050] = 392
 scopes data    [0x000000010dcd9050,0x000000010dcd9648] = 1528
 scopes pcs     [0x000000010dcd9648,0x000000010dcd98d8] = 656
 dependencies   [0x000000010dcd98d8,0x000000010dcd9918] = 64
 handler table  [0x000000010dcd9918,0x000000010dcd9a98] = 384
 nul chk table  [0x000000010dcd9a98,0x000000010dcd9ae8] = 80
Compiled method (c2)   56042 16441   !   4       java.lang.invoke.MethodHandleImpl::makeGuardWithTest (159 bytes)
 total in heap  [0x000000010dcd7610,0x000000010dcd9ae8] = 9432
 relocation     [0x000000010dcd7768,0x000000010dcd79f0] = 648
 main code      [0x000000010dcd7a00,0x000000010dcd8a40] = 4160
 stub code      [0x000000010dcd8a40,0x000000010dcd8e68] = 1064
 oops           [0x000000010dcd8e68,0x000000010dcd8ec8] = 96
 metadata       [0x000000010dcd8ec8,0x000000010dcd9050] = 392
 scopes data    [0x000000010dcd9050,0x000000010dcd9648] = 1528
 scopes pcs     [0x000000010dcd9648,0x000000010dcd98d8] = 656
 dependencies   [0x000000010dcd98d8,0x000000010dcd9918] = 64
 handler table  [0x000000010dcd9918,0x000000010dcd9a98] = 384
 nul chk table  [0x000000010dcd9a98,0x000000010dcd9ae8] = 80
Compiled method (c2)   56043 16441   !   4       java.lang.invoke.MethodHandleImpl::makeGuardWithTest (159 bytes)
 total in heap  [0x000000010dcd7610,0x000000010dcd9ae8] = 9432
 relocation     [0x000000010dcd7768,0x000000010dcd79f0] = 648
 main code      [0x000000010dcd7a00,0x000000010dcd8a40] = 4160
 stub code      [0x000000010dcd8a40,0x000000010dcd8e68] = 1064
 oops           [0x000000010dcd8e68,0x000000010dcd8ec8] = 96
 metadata       [0x000000010dcd8ec8,0x000000010dcd9050] = 392
 scopes data    [0x000000010dcd9050,0x000000010dcd9648] = 1528
 scopes pcs     [0x000000010dcd9648,0x000000010dcd98d8] = 656
 dependencies   [0x000000010dcd98d8,0x000000010dcd9918] = 64
 handler table  [0x000000010dcd9918,0x000000010dcd9a98] = 384
 nul chk table  [0x000000010dcd9a98,0x000000010dcd9ae8] = 80
Compiled method (c2)   56043 16441   !   4       java.lang.invoke.MethodHandleImpl::makeGuardWithTest (159 bytes)
 total in heap  [0x000000010dcd7610,0x000000010dcd9ae8] = 9432
 relocation     [0x000000010dcd7768,0x000000010dcd79f0] = 648
 main code      [0x000000010dcd7a00,0x000000010dcd8a40] = 4160
 stub code      [0x000000010dcd8a40,0x000000010dcd8e68] = 1064
 oops           [0x000000010dcd8e68,0x000000010dcd8ec8] = 96
 metadata       [0x000000010dcd8ec8,0x000000010dcd9050] = 392
 scopes data    [0x000000010dcd9050,0x000000010dcd9648] = 1528
 scopes pcs     [0x000000010dcd9648,0x000000010dcd98d8] = 656
 dependencies   [0x000000010dcd98d8,0x000000010dcd9918] = 64
 handler table  [0x000000010dcd9918,0x000000010dcd9a98] = 384
 nul chk table  [0x000000010dcd9a98,0x000000010dcd9ae8] = 80
#
# If you would like to submit a bug report, please visit:
#   https://aka.ms/minecraftjavacrashes
#"#;
        let error_info = JreFatalError::parse(text).unwrap();
        dbg!(error_info);
    }

    #[test]
    fn example_6() {
        let text = r#"#
# A fatal error has been detected by the Java Runtime Environment:
#
#  EXCEPTION_ACCESS_VIOLATION (0xc0000005) at pc=0x0000021d40d9fd20, pid=30236, tid=3276
#
# JRE version: OpenJDK Runtime Environment Microsoft-11369940 (21.0.7 6) (build 21.0.7 6-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-11369940 (21.0.7 6-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, windows-amd64)
# Problematic frame:
# C  0x0000021d40d9fd20
#
# No core dump will be written. Minidumps are not enabled by default on client versions of Windows
#
# An error report file with more information is saved as:
# C:\Users\********\AppData\Roaming\PrismLauncher\instances\1.21.7\minecraft\hs_err_pid30236.log
[1530.213s][warning][os] Loading hsdis library failed
#
# If you would like to submit a bug report, please visit:
#   https://aka.ms/minecraftjavacrashes
# The crash happened outside the Java Virtual Machine in native code.
# See problematic frame for where to report the bug.
#"#;
        let error_info = JreFatalError::parse(text).unwrap();
        dbg!(error_info);
    }

    #[test]
    fn example_7() {
        let text = r#"#
# A fatal error has been detected by the Java Runtime Environment:
#
#  Internal Error (os_windows_x86.cpp:144), pid=14504, tid=4472
#  guarantee(result == EXCEPTION_CONTINUE_EXECUTION) failed: Unexpected result from topLevelExceptionFilter
#
# JRE version: OpenJDK Runtime Environment Microsoft-9388422 (21.0.3 9) (build 21.0.3 9-LTS)
# Java VM: OpenJDK 64-Bit Server VM Microsoft-9388422 (21.0.3 9-LTS, mixed mode, tiered, compressed oops, compressed class ptrs, g1 gc, windows-amd64)
# No core dump will be written. Minidumps are not enabled by default on client versions of Windows
#
# An error report file with more information is saved as:
# C:\Users\********\AppData\Roaming\PrismLauncher\instances\1.21.5\minecraft\hs_err_pid14504.log
#
# If you would like to submit a bug report, please visit:
#   https://aka.ms/minecraftjavacrashes
# The crash happened outside the Java Virtual Machine in native code.
# See problematic frame for where to report the bug.
#"#;
        let error_info = JreFatalError::parse(text).unwrap();
        dbg!(error_info);
    }
}
