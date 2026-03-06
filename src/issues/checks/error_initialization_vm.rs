use crate::{header::index::IndexedLogHeader, issues::issue::Issue};


pub(crate) fn error_initialization_vm(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    for line in header.text.lines() {
        if line.starts_with("Error occurred during initialization of VM") {
            return Some(Issue::ErrorInitializationVM)
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_encoding() {
       let header_fragment = r#"Minecraft process ID: 179


Error occurred during initialization of VM
java.lang.InternalError: platform encoding not initialized
	at jdk.internal.util.SystemProps$Raw.platformProperties(java.base/Native Method)
	at jdk.internal.util.SystemProps$Raw.<init>(java.base/SystemProps.java:263)
	at jdk.internal.util.SystemProps.initProperties(java.base/SystemProps.java:67)
	at java.lang.System.initPhase1(java.base/System.java:2162)
Process exited with code 1.
Log upload triggered at: 17 Jan 2026 10:00:34 -0600
"#;
        let indexed_header = IndexedLogHeader::index_header(header_fragment);
        let issue = error_initialization_vm(&indexed_header).expect("Failed to detect issue");
        let Issue::ErrorInitializationVM = issue else { panic!("Not the right issue"); };
    }

    #[test]
    fn boot_class_path() {
       let header_fragment = r#"Minecraft process ID: 20004


Error occurred during initialization of VM
Failed setting boot class path.
Process exited with code 1.
"#;
        let indexed_header = IndexedLogHeader::index_header(header_fragment);
        let issue = error_initialization_vm(&indexed_header).expect("Failed to detect issue");
        let Issue::ErrorInitializationVM = issue else { panic!("Not the right issue"); };
    }
}