use crate::{header::index::IndexedLogHeader, issues::issue::Issue};


pub(crate) fn instance_update_failed(header: &IndexedLogHeader<'_>) -> Option<Issue> {
    for line in header.text.lines() {
        if let Some(reason) = line.strip_prefix("Instance update failed because: ") {
            return Some(Issue::InstanceUpdateFailed(reason.to_string()))
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_metadata() {
       let header_fragment = r#"

Instance update failed because: Component metadata update task failed while downloading from remote server:
One or more subtasks failed
One or more subtasks failed
"#;
        let indexed_header = IndexedLogHeader::index_header(header_fragment);
        let issue = instance_update_failed(&indexed_header).expect("Failed to detect issue");
        let Issue::InstanceUpdateFailed(reason) = issue else { panic!("Not the right issue"); };
        assert_eq!(reason, "Component metadata update task failed while downloading from remote server:");
    }

    #[test]
    fn game_update_failed() {
       let header_fragment = r#"
Instance update failed because: Game update failed: it was impossible to fetch the required libraries.
Reason:
Multiple subtasks failed
HTTP/2 protocol error
HTTP/2 protocol error
"#;
        let indexed_header = IndexedLogHeader::index_header(header_fragment);
        let issue = instance_update_failed(&indexed_header).expect("Failed to detect issue");
        let Issue::InstanceUpdateFailed(reason) = issue else { panic!("Not the right issue"); };
        assert_eq!(reason, "Game update failed: it was impossible to fetch the required libraries.");
    }
}