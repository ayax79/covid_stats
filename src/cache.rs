use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Result as IoResult, Error as IoError};

const US_CSV: &str = "us.csv";

struct Cache<'a> {
    cache_dir: &'a Path, 
}

impl <'a> Cache<'a> {

    pub fn new(cache_dir: &'a Path) -> IoResult<Self> {
        if !cache_dir.exists() {
            fs::create_dir(cache_dir)?;
        }
        Ok(Cache {
            cache_dir,
        })
    }

    pub fn us(&self) -> PathBuf {
        self.cache_dir.join(US_CSV)
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile;

    #[test]
    fn test_new_cache() {
        let tmpdir = tempfile::tempdir().unwrap();
        let cache_dir = tmpdir.path().join("cache");
        assert!(!cache_dir.exists());
        let _ = Cache::new(&cache_dir).unwrap();
        assert!(cache_dir.exists());
    }
}