use std::io::{Error as IoError, Result as IoResult, ErrorKind as IoErrorKind};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::BufWriter;

const US_CSV: &str = "us.csv";

#[derive(Clone)]
pub struct Cache {
    cache_dir: PathBuf,
}

impl Cache {
    pub fn new(cache_dir: &Path) -> IoResult<Self> {
        if !cache_dir.exists() {
            fs::create_dir(cache_dir)?;
        }
        Ok(Cache { cache_dir: cache_dir.to_owned() })
    }

    pub fn us(&self) -> PathBuf {
        self.cache_dir.join(US_CSV)
    }

    pub(crate) fn us_wtr(&self) -> IoResult<BufWriter<File>> {
        let path= self.us();
        build_writer(&path)
    }
}

fn build_writer(path: &Path) -> IoResult<BufWriter<File>> {
    backup(path)?;
    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}

/// Backs up the specified path
fn backup(path: &Path) -> IoResult<()> {
    if path.exists() {
        let path_str = path.to_str().ok_or(IoError::new(IoErrorKind::Other, "invalid string"))?;
        let backup_path = path_str.to_owned() + ".bak";
        fs::rename(path, backup_path)?;
    }
    Ok(())
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
