use crate::issues::issue::Issue;

fn java_32_bit(text: &str) -> Option<Issue> {
    let found = text.contains("Could not reserve enough space for ")
		|| text.contains("Invalid maximum heap size: ")
		|| text.contains("Invalid initial heap size: ");
	found.then_some(Issue::Java32Bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_java_32_bit() {
        let text = "There is not enough space on the disk
Could not reserve enough space for object heap";
        let issue = java_32_bit(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::Java32Bit);
    }
}