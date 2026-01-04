use crate::issues::issue::Issue;

pub(crate) fn java_32_bit(text: &str) -> Option<Issue> {
    let found = text.contains("Could not reserve enough space for ")
		|| text.contains("Invalid maximum heap size: ")
		|| text.contains("Invalid initial heap size: ");
	found.then_some(Issue::Java32BitMemoryLimit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_java_32_bit() {
        let text = r#"Minecraft process ID: 8252


Error occurred during initialization of VM
Could not reserve enough space for 2703360KB object heap
Process exited with code 1.
"#;
        let issue = java_32_bit(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::Java32BitMemoryLimit);
    }
}