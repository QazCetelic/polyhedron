use crate::issues::issue::Issue;

fn forge_missing_dependencies(entry_text: &str) -> Option<Issue> {
    entry_text.contains("Missing or unsupported mandatory dependencies").then_some(Issue::ForgeMissingDependencies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_forge_missing_dependencies() {
        let text = r"[02:55:39] [ForkJoinPool.commonPool-worker-1/INFO] [ne.ne.fm.lo.mo.lo.JarInJarDependencyLocator/]: Found 5 dependencies adding them to mods collection
[02:55:39] [ForkJoinPool.commonPool-worker-1/INFO] [ne.ne.fm.lo.mo.ModDiscoverer/]: 
     Mod List:
		Name Version (Mod Id)

		Iris 1.9.6+mc1.21.10 (iris)
		Minecraft 1.21.10 (minecraft)
		NeoForge 21.10.8-beta (neoforge)
[02:55:39] [ForkJoinPool.commonPool-worker-1/ERROR] [ne.ne.fm.lo.ModSorter/LOADING]: Missing or unsupported mandatory dependencies:
	Mod ID: 'sodium', Requested by: 'iris', Expected range: '[0.6,)', Actual version: '[MISSING]'
[02:55:39] [background-scan-handler-1/ERROR] [ne.ne.fm.lo.mo.BackgroundScanHandler/SCAN]: An error occurred scanning file Jar[C:\Users\danil\AppData\Roaming\PrismLauncher\instances\1.21.10\minecraft\mods\iris-neoforge-1.9.6+mc1.21.10.jar]
java.util.concurrent.CompletionException: java.lang.IllegalStateException: zip file closed
	at java.base/java.util.concurrent.CompletableFuture.encodeThrowable(CompletableFuture.java:315) ~[?:?] {}
	at java.base/java.util.concurrent.CompletableFuture.completeThrowable(CompletableFuture.java:320) ~[?:?] {}
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1770) ~[?:?] {}
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1144) ~[?:?] {}
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:642) ~[?:?] {}
	at java.base/java.lang.Thread.run(Thread.java:1583) [?:?] {}
Caused by: java.lang.IllegalStateException: zip file closed
	at java.base/java.util.zip.ZipFile.ensureOpen(ZipFile.java:846) ~[?:?] {}
	at java.base/java.util.zip.ZipFile.jarEntries(ZipFile.java:547) ~[?:?] {}
	at java.base/java.util.zip.ZipFile$1.entries(ZipFile.java:1137) ~[?:?] {}
	at java.base/java.util.jar.JarFile.entries(JarFile.java:529) ~[?:?] {}
	at net.neoforged.fml.jarcontents.JarFileContents.visitContent(JarFileContents.java:148) ~[loader-10.0.14.jar:10.0] {}
	at net.neoforged.fml.jarcontents.JarContents.visitContent(JarContents.java:215) ~[loader-10.0.14.jar:10.0] {}
	at net.neoforged.fml.loading.modscan.Scanner.scan(Scanner.java:27) ~[loader-10.0.14.jar:10.0] {}
	at net.neoforged.fml.loading.moddiscovery.ModFile.lambda$startScan$5(ModFile.java:155) ~[loader-10.0.14.jar:10.0] {}
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1768) ~[?:?] {}
	... 3 more
";
        let issue = forge_missing_dependencies(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::ForgeMissingDependencies);
    }
}