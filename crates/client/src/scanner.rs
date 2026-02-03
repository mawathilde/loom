use crate::config::get_config;
use loom_core::hash::hash_file;
use loom_core::manifest::GameManifest;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_game_dir(game_id: String, base: &Path) -> GameManifest {
    let device_id = get_config().unwrap().device_id;

    let mut files = Vec::new();

    for entry in WalkDir::new(base) {
        let entry = entry.unwrap();

        if should_ignore(&entry) {
            continue;
        }

        let path = entry
            .path()
            .strip_prefix(base)
            .unwrap()
            .to_string_lossy()
            .replace("\\", "/");

        let metadata = entry.metadata().unwrap();
        let hash = hash_file(entry.path()).unwrap().to_hex().to_string();
        let modified_at = metadata
            .modified()
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let size = metadata.len();

        files.push(loom_core::manifest::FileEntry {
            path,
            hash,
            modified_at,
            size,
        });
    }

    GameManifest {
        game_id,
        device_id,
        files,
        generated_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

fn should_ignore(entry: &walkdir::DirEntry) -> bool {
    if entry.path().is_dir() {
        return true;
    }

    let file_name = entry.file_name().to_string_lossy();
    if file_name.starts_with('.') {
        return true;
    }

    false
}
