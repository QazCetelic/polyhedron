use crate::issues::issue::Issue;

fn invalid_proxy(text: &str) -> Option<Issue> {
    text.contains("Connection to proxy refused").then_some(Issue::InvalidProxy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_invalid_proxy() {
        let text = r#"     0.323 D | [launcher.instance.profile.resolve]: "Remarkably Optimized 1.14.3" | Remote loading is being run for "LWJGL 3"
     0.327 D | [launcher.instance.profile.resolve]: "Remarkably Optimized 1.14.3" | Remote loading is being run for "Minecraft"
     0.328 D | [launcher.instance.profile.resolve]: "Remarkably Optimized 1.14.3" | Remote loading is being run for "Intermediary Mappings"
     0.329 D | [launcher.instance.profile.resolve]: "Remarkably Optimized 1.14.3" | Remote loading is being run for "Fabric Loader"
     0.329 D | [launcher.instance.profile]: "Remarkably Optimized 1.14.3" | Component list update/resolve task failed  Reason: "Some component metadata load tasks failed."
     0.342 D | Instance-type specific settings were loaded!
     0.364 D | <> Main window shown.
     0.368 W | [qt.networkauth.oauth2]: Token request failed: "Connection to proxy refused"
     0.368 W | "Failed to refresh token."
     0.368 D | RefreshSchedule: Background account refresh failed:  "Failed to refresh token."
     0.368 D | Dir changed: "/home/********/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/translations"
     0.369 C | [launcher.task.net.download]: "{88d12a72-aa17-44b4-9930-2577502a641c}" Failed "https://i18n.prismlauncher.org/index_v2.json" with reason QNetworkReply::ProxyConnectionRefusedError
     0.369 C | [launcher.task.net.download]: "{88d12a72-aa17-44b4-9930-2577502a641c}" HTTP Status 0 ;error "Connection to proxy refused"
"#;
        let issue = invalid_proxy(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::InvalidProxy);
    }
}