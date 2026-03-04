pub struct McVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: Option<u8>,
}

impl McVersion {
    pub fn from_str(s: &str) -> Option<McVersion> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 3 { return None; }
        let major: u8 = parts.get(0)?.parse::<u8>().ok()?;
        let minor: u8 = parts.get(1)?.parse::<u8>().ok()?;
        let patch: Option<u8> = if let Some(patch_str) = parts.get(2) { Some(patch_str.parse::<u8>().ok()?) } else { None };
        Some(McVersion { major, minor, patch })
    }
}