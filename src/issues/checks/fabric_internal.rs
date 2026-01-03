// All collected test data that contained matched this error, also contained "org.lwjgl.LWJGLException: Could not choose GLX13 config" right after it, TODO find correlation

use crate::issues::issue::Issue;

fn fabric_internal(text: &str) -> Option<Issue> {
    const CLASS_NOT_FOUND: &str = "Caused by: java.lang.ClassNotFoundException: ";
    let errors = [
		&format!("{CLASS_NOT_FOUND}net.fabricmc.fabric.impl"),
		&format!("{CLASS_NOT_FOUND}net.fabricmc.fabric.mixin"),
		&format!("{CLASS_NOT_FOUND}net.fabricmc.fabric.loader.impl"),
		&format!("{CLASS_NOT_FOUND}net.fabricmc.fabric.loader.mixin"),
		"org.quiltmc.loader.impl.FormattedException: java.lang.NoSuchMethodError:",
	];

    if errors.iter().any(|e| text.contains(e)) {
        Some(Issue::FabricInternalAccess)
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_fabric_internal() {
        let text = "Caused by: java.lang.UnsatisfiedLinkError: org/lwjgl/opengl/LinuxDisplay
Caused by: java.lang.ClassNotFoundException: net.fabricmc.fabric.impl
java.lang.NoSuchMethodError: sun.security.util.ManifestEntryVerifier.<init>(Ljava/util/jar/Manifest;)V";

        let issue = fabric_internal(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::FabricInternalAccess);
    }
}