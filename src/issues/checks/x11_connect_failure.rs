use crate::issues::issue::Issue;

fn x11_connect_failure(text: &str) -> Option<Issue> {
    let found = text.contains("Can't connect to X11 window server using ':0.0' as the value of the DISPLAY variable")
        || text.contains("Could not open X display connection");
	found.then_some(Issue::X11ConnectFailure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_not_connect_using_0_0() {
        let text = r#"Caused by: java.awt.AWTError: Can't connect to X11 window server using ':0.0' as the value of the DISPLAY variable."#;
        let issue = x11_connect_failure(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::X11ConnectFailure);
    }

    #[test]
    fn can_not_open_x_display_connection() {
        let text = r#"Caused by: org.lwjgl.LWJGLException: Could not open X display connection"#;
        let issue = x11_connect_failure(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::X11ConnectFailure);
    }
}