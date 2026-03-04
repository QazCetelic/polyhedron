use crate::{issues::issue::Issue, parse::{crash_report::CrashReport, stacktrace::model::Stacktrace}};

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntrypointExecutionErrors {
    pub method: String,
    pub mod_name: String,
    pub class_name: String,
}

pub(crate) fn entrypoint_execution_errors(stacktraces: &[Stacktrace]) -> Option<Issue> {
    // Could not execute entrypoint stage 'main' due to errors, provided by 'create' at 'com.simibubi.create.Create'!
    let msg = &stacktraces.first()?.message;
    let msg = msg.strip_prefix("Could not execute entrypoint stage '")?.strip_suffix("'!")?;
    let (method, rest) = msg.split_once("' due to errors, provided by '")?;
    let (mod_name, class_name) = rest.split_once("' at '")?;

    let errors = EntrypointExecutionErrors {
        method: method.to_string(),
        mod_name: mod_name.to_string(),
        class_name: class_name.to_string(),
    };

    Some(Issue::EntrypointExecutionErrors(Box::new(errors)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find() {
        let crash_report = include_str!("../../parse/test_data/crash_2.log");
        let report = CrashReport::parse(crash_report).expect("Failed to parse crash report");
        let issue = entrypoint_execution_errors(&report.stacktrace).expect("Failed to find issue");
        let Issue::EntrypointExecutionErrors(errors) = issue else { panic!("Not the right issue"); };
        assert_eq!(errors.method, "client");
        assert_eq!(errors.mod_name, "betteradvancements");
        assert_eq!(errors.class_name, "betteradvancements.fabric.BetterAdvancements");
    }
}