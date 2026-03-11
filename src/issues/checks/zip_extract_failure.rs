use crate::{issues::issue::Issue, parse::stacktrace::model::Stacktrace};

pub(crate) fn zip_extract_failure(stacktraces: &[Stacktrace]) -> Option<Issue> {
    for stacktrace in stacktraces {
        if stacktrace.exception == "java.util.zip.ZipException" || stacktrace.message.starts_with("java.util.zip.ZipException") {
            return Some(Issue::ZipExtractFailure);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::entries::entry::LogEntry;
    use super::*;

    #[test]
    fn example_1() {
        let log = r#"[23:17:37] [main/ERROR] [FML]: Unable to read the jar file CodeChickenLib-1.7.10-1.1.3.138-universal.jar - ignoring
java.util.zip.ZipException: error in opening zip file
	at java.util.zip.ZipFile.open(Native Method) ~[?:1.8.0_202]
	at java.util.zip.ZipFile.<init>(ZipFile.java:225) ~[?:1.8.0_202]
	at java.util.zip.ZipFile.<init>(ZipFile.java:155) ~[?:1.8.0_202]
	at java.util.jar.JarFile.<init>(JarFile.java:166) ~[?:1.8.0_202]
	at java.util.jar.JarFile.<init>(JarFile.java:130) ~[?:1.8.0_202]"#;
        let entries = LogEntry::from_lines(log.lines());
        let entry = entries.first().expect("Failed to get entry");
        let stacktraces: Vec<Stacktrace> = Stacktrace::from_lines(entry.contents.lines()).collect();
        let issue = zip_extract_failure(&stacktraces).expect("Failed to determine issue");
        assert_eq!(issue, Issue::ZipExtractFailure);
    }

    #[test]
    fn example_2() {
        let log = r#"[12:23:01] [Client thread/WARN] [reccomplex]: Error reading structure
ivorius.reccomplex.structures.generic.StructureLoadException: java.util.zip.ZipException: invalid entry size (expected 930 but got 928 bytes)
	at ivorius.reccomplex.structures.generic.StructureSaveHandler.structureInfoFromZip(StructureSaveHandler.java:215) ~[StructureSaveHandler.class:?]
	at ivorius.reccomplex.structures.generic.StructureSaveHandler.readGenericStructure(StructureSaveHandler.java:162) ~[StructureSaveHandler.class:?]
	at ivorius.reccomplex.structures.generic.StructureSaveHandler.loadFile(StructureSaveHandler.java:224) [StructureSaveHandler.class:?]
	at ivorius.reccomplex.files.FileTypeRegistry.tryLoad(FileTypeRegistry.java:121) [FileTypeRegistry.class:?]"#;
        let entries = LogEntry::from_lines(log.lines());
        let entry = entries.first().expect("Failed to get entry");
        let stacktraces: Vec<Stacktrace> = Stacktrace::from_lines(entry.contents.lines()).collect();
        assert_eq!(stacktraces.len(), 1);
        let issue = zip_extract_failure(&stacktraces).expect("Failed to determine issue");
        assert_eq!(issue, Issue::ZipExtractFailure);
    }

    #[test]
    fn example_3() {
        let log = r#"[17:16:15] [main/WARN] [gg.es.lo.st.EssentialLoaderBase/]: Found newer Essential version 1.3.9.1 [stable], skipping at user request
Exception caught from launcher
java.lang.reflect.InvocationTargetException
	at java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
	at java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:77)
	at java.base/jdk.internal.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
	at java.base/java.lang.reflect.Method.invoke(Method.java:569)
	at io.github.zekerzhayard.forgewrapper.installer.Main.main(Main.java:69)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
Caused by: cpw.mods.niofs.union.UnionFileSystem$UncheckedIOException: java.util.zip.ZipException: zip END header not found
Caused by: java.util.zip.ZipException: zip END header not found
	at jdk.zipfs/jdk.nio.zipfs.ZipFileSystem.findEND(ZipFileSystem.java:1320)
	at jdk.zipfs/jdk.nio.zipfs.ZipFileSystem.initCEN(ZipFileSystem.java:1534)"#;
        let entries = LogEntry::from_lines(log.lines());
        let entry = entries.first().expect("Failed to get entry");
        let stacktraces: Vec<Stacktrace> = Stacktrace::from_lines(entry.contents.lines()).collect();
        assert_eq!(stacktraces.len(), 1);
        let issue = zip_extract_failure(&stacktraces).expect("Failed to determine issue");
        assert_eq!(issue, Issue::ZipExtractFailure);
    }
}