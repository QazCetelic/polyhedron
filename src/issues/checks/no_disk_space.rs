use crate::{header::extract::LibraryInfo, issues::issue::Issue};

fn no_disk_space(entry_text: &str) -> Option<Issue> {
    entry_text.contains("There is not enough space on the disk").then_some(Issue::NoDiskSpace)
}

#[cfg(test)]
mod tests {
    use crate::header::index::IndexedLogHeader;

    use super::*;

    #[test]
    fn matches_no_disk_space() {
        let text = r#"[21:27:46] [Thread-77/INFO] [xa.ma.WorldMap/]: Retrying...
[21:27:47] [Thread-77/ERROR] [xa.ma.WorldMap/]: IO exception while trying to save (2147483647) 0_-1 L0 xaero.map.region.MapRegion@8244321
java.io.IOException: There is not enough space on the disk
	at java.io.FileOutputStream.writeBytes(Native Method) ~[?:?] {}
	at java.io.FileOutputStream.write(FileOutputStream.java:349) ~[?:?] {}
	at java.io.BufferedOutputStream.flushBuffer(BufferedOutputStream.java:81) ~[?:?] {}
	at java.io.BufferedOutputStream.write(BufferedOutputStream.java:127) ~[?:?] {}
	at java.util.zip.DeflaterOutputStream.deflate(DeflaterOutputStream.java:261) ~[?:?] {}
	at java.util.zip.ZipOutputStream.closeEntry(ZipOutputStream.java:268) ~[?:?] {}
	at java.util.zip.ZipOutputStream.finish(ZipOutputStream.java:374) ~[?:?] {}
	at java.util.zip.DeflaterOutputStream.close(DeflaterOutputStream.java:244) ~[?:?] {}
	at java.util.zip.ZipOutputStream.close(ZipOutputStream.java:391) ~[?:?] {}
	at java.io.FilterOutputStream.close(FilterOutputStream.java:191) ~[?:?] {}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:540) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.file.MapSaveLoad.run(MapSaveLoad.java:1090) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.MapProcessor.run(MapProcessor.java:344) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at xaero.map.MapRunner.run(MapRunner.java:18) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
	at java.lang.Thread.run(Thread.java:833) ~[?:?] {re:mixin}
	Suppressed: java.io.IOException: There is not enough space on the disk
		at java.io.FileOutputStream.writeBytes(Native Method) ~[?:?] {}
		at java.io.FileOutputStream.write(FileOutputStream.java:349) ~[?:?] {}
		at java.io.BufferedOutputStream.flushBuffer(BufferedOutputStream.java:81) ~[?:?] {}
		at java.io.BufferedOutputStream.flush(BufferedOutputStream.java:142) ~[?:?] {}
		at java.util.zip.DeflaterOutputStream.flush(DeflaterOutputStream.java:290) ~[?:?] {}
		at java.io.DataOutputStream.flush(DataOutputStream.java:128) ~[?:?] {}
		at java.io.FilterOutputStream.close(FilterOutputStream.java:182) ~[?:?] {}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:540) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.saveRegion(MapSaveLoad.java:563) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.file.MapSaveLoad.run(MapSaveLoad.java:1090) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.MapProcessor.run(MapProcessor.java:344) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at xaero.map.MapRunner.run(MapRunner.java:18) ~[XaerosWorldMap_1.39.12_Forge_1.20.jar#439!/:1.39.12] {re:classloading}
		at java.lang.Thread.run(Thread.java:833) ~[?:?] {re:mixin}
[21:27:47] [Thread-77/INFO] [xa.ma.WorldMap/]: Retrying...
"#;
        let issue = no_disk_space(&text).expect("Failed to determine issue");
        assert_eq!(issue, Issue::NoDiskSpace);
    }
}