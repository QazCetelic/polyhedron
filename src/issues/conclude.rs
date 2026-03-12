use std::collections::{BTreeMap, BTreeSet};

use crate::issues::issue::Issue;

pub fn collect_problematic_mods(issues: &[Issue], mod_name_lookup: Option<BTreeMap<String, String>>) -> BTreeSet<String> {
    let mut mods: BTreeSet<String> = BTreeSet::new();

    fn insert_normalized(normalized_name: &str, mods: &mut BTreeSet<String>, mod_name_lookup: &Option<BTreeMap<String, String>>) {
        if let Some(mod_name) = mod_name_lookup.as_ref().map(|map| map.get(normalized_name)).flatten() {
            mods.insert(mod_name.clone());
        }
        else {
            mods.insert(normalized_name.to_string()); // Insert normalized name as fallback
        }
    }

    for issue in issues {
        match issue {
            // Almost certain
            Issue::EntrypointExecutionErrors(entrypoint_execution_errors) => {
                insert_normalized(&entrypoint_execution_errors.normalized_mod_name, &mut mods, &mod_name_lookup);
                return mods; // Stop
            },
            Issue::CriticalInjectionFailure(critical_injection_failure) => {
                insert_normalized(&critical_injection_failure.normalized_mod_name, &mut mods, &mod_name_lookup);
                return mods; // Stop
            },
            Issue::MixinApplyFailure(normalized_mod_name) => {
                mods.insert(normalized_mod_name.clone());
            },
            Issue::ForgeSuspectedMod(suspected_mod_infos) => {
                for mod_info in suspected_mod_infos {
                    mods.insert(mod_info.mod_name.clone());
                }
            },
            Issue::ModsFoundInStacktraceNamespace(mod_names) => {
                for mod_name in mod_names {
                    mods.insert(mod_name.clone());
                }
            },
            Issue::ModsFoundInStacktraceInfo(mod_names) => {
                for mod_name in mod_names {
                    mods.insert(mod_name.clone());
                }
            },
            Issue::IncompatibleMods(_incompatible_mods_info) => {
                // TODO consider adding
            },
            _ => {},
        }
    }
    mods
}


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RecommendedJavaVersion {
    pub major: Option<u8>,
    pub minor: Option<u16>,
}

pub fn recommend_java_version(issues: &[Issue]) -> Option<RecommendedJavaVersion> {
    for issue in issues {
        match issue {
            Issue::ForgeJava => { return Some(RecommendedJavaVersion { major: Some(8), minor: Some(312) }) /* 8u312 or lower */ },
            Issue::Lwjgl2JavaAbove8 => { return Some(RecommendedJavaVersion { major: Some(8), minor: None }) /* Java 8 or Temurin */ },
            Issue::NettyJavaAbove8 => { return Some(RecommendedJavaVersion { major: Some(8), minor: None }) },
            Issue::WrongJava(version) => { if let Some(ver) = version { return Some(RecommendedJavaVersion { major: Some(*ver), minor: None }) } },
            _ => {},
        }
    }
    None
}