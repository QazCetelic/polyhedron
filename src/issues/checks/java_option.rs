use lazy_regex::regex;

use crate::issues::issue::Issue;

fn java_option(text: &str) -> Option<Issue> {
    let vm_option_regex = regex!(r"Unrecognized VM option '(.+)'[\r\n]");
    let unrecognized_option_regex = regex!(r"Unrecognized option: (.+)[\r\n]");

    if let Some(captures) = vm_option_regex.captures(text) {
        let arg = format!("-XX:{}", &captures[1]);
        dbg!(&arg);
        return Some(Issue::JavaOption(arg));
	}
	else if let Some(captures) = unrecognized_option_regex.captures(text) {
        let arg = captures[1].to_string();
        return Some(Issue::JavaOption(arg));
	}

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_option_shenandoah() {
        let text = "Unrecognized VM option 'UseShenandoahGC'\n";
        let issue = java_option(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::JavaOption("-XX:UseShenandoahGC".to_string()));
    }

    #[test]
    fn vm_option_zgc() {
        let text = "Unrecognized VM option 'UseZGC'\n";
        let issue = java_option(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::JavaOption("-XX:UseZGC".to_string()));
    }
}