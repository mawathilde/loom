pub struct FileEntry {
    pub path: String,
    pub hash: String,
    pub modified_at: u64,
    pub size: u64,
}

pub struct GameManifest {
    pub game_id: String,
    pub device_id: String,
    pub files: Vec<FileEntry>,
    pub generated_at: u64,
}
