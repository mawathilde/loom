use blake3::Hash;

pub fn hash(bytes: &[u8]) -> Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(bytes);
    hasher.finalize()
}

pub fn hash_file(path: &std::path::Path) -> std::io::Result<Hash> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_basic() {
        let data = b"Hello, world!";
        let hash = hash(data).to_hex().to_string();
        assert_eq!(
            hash,
            "ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d"
        );
    }

    #[test]
    fn test_hash_empty() {
        let data = b"";
        let hash = hash(data).to_hex().to_string();
        assert_eq!(
            hash,
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
        let hash1 = hash(data);
        let hash2 = hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_file_basic() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(&tmp, b"Hello, file!").unwrap();

        let hash = hash_file(tmp.path()).unwrap().to_hex().to_string();
        assert_eq!(
            hash,
            "7cbb4363d5749995b3891f5d0699036ab788215fcbdaa5203d33ec7173e6da3f"
        );
    }

    #[test]
    fn test_hash_file_empty() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let hash = hash_file(tmp.path()).unwrap().to_hex().to_string();
        assert_eq!(
            hash,
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn test_hash_file_not_found() {
        let result = hash_file(std::path::Path::new("/nonexistent/file"));
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_consistency_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(&tmp, b"consistent data").unwrap();

        let hash1 = hash_file(tmp.path()).unwrap();
        let hash2 = hash_file(tmp.path()).unwrap();
        assert_eq!(hash1, hash2);
    }
}
