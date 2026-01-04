use crate::{entries::entry::LogEntry, issues::issue::Issue};

pub(crate) fn macos_ns(entry: &LogEntry) -> Option<Issue> {
    entry.contents.contains("Terminating app due to uncaught exception 'NSInternalInconsistencyException").then_some(Issue::MacOSNSInternal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_macos_ns() {
        // Text starts after header
        let text = "[20:48:10] [VersionCheck/INFO] [Config]: [OptiFine] Checking for new version2022-12-27 20:48:10.200 java[3916:64291] *** Terminating app due to uncaught exception 'NSInternalInconsistencyException', reason: 'NSWindow drag regions should only be invalidated on the Main Thread!'*** First throw call stack:(    0   CoreFoundation                      0x00007ff816989e9b __exceptionPreprocess + 242    1   libobjc.A.dylib                     0x00007ff8166ebe48 objc_exception_throw + 48    2   CoreFoundation                      0x00007ff8169b28c6 -[NSException raise] + 9    3   AppKit                              0x00007ff8192a60a4 -[NSWindow(NSWindow_Theme) _postWindowNeedsToResetDragMarginsUnlessPostingDisabled] + 321    4   AppKit                              0x00007ff8192cd6ab -[NSThemeFrame setStyleMask:] + 162    5   AppKit                              0x00007ff8192cd4bb __25-[NSWindow setStyleMask:]_block_invoke + 2092    6   AppKit                              0x00007ff8192ccc37 NSPerformVisuallyAtomicChange + 132    7   AppKit                              0x00007ff8192ccb44 -[NSWindow setStyleMask:] + 170    8   liblwjgl.dylib                      0x000000016473acca Java_org_lwjgl_opengl_MacOSXDisplay_nSetResizable + 90    9   ???                                 0x00000001101e7187 0x0 + 4565397895    10  ???                                 0x00000001101d733d 0x0 + 4565332797)libc++abi: terminating with uncaught exception of type NSExceptionProcess crashed with exitcode 6.";
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries.iter().filter_map(|e| macos_ns(e)).next().expect("Failed to determine issue");
        assert_eq!(issue, Issue::MacOSNSInternal);
    }
}