use reqwest::Client;
use std::io::{self, BufWriter, Write};
use std::fs::File;
use std::path::PathBuf;

use crate::cache::Cache;

pub(crate) const US_DATA: &str =
    "https://raw.githubusercontent.com/nytimes/covid-19-data/master/us.csv";

pub struct HttpClient {
    client: Client,
    cache: Cache,
}

impl HttpClient {
    pub fn new(cache: &Cache) -> Self {
        HttpClient {
            client: Client::new(),
            cache: cache.to_owned(),
        }
    }

    pub async fn fetch_us(&self) -> io::Result<PathBuf> {
        let mut wtr = self.cache.us_wtr()?;
        self.fetch_raw(US_DATA, &mut wtr).await?;
        Ok(self.cache.us())
    } 

    async fn fetch_raw(&self, url: &str, wtr: &mut BufWriter<File>) -> io::Result<()> {
        let mut rs = self.client.get(url).send().await.map_err(as_ioerror)?;

        while let Some(chunk) = rs.chunk().await.map_err(as_ioerror)? {
            wtr.write(&chunk)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

fn as_ioerror(e: reqwest::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile;

    #[tokio::test]
    async fn test_fetch_us() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let cache = Cache::new(tmp_dir.path()).unwrap();
        let client = HttpClient::new(&cache);
        assert!(!cache.us().exists());
        let file_path = client.fetch_us().await.unwrap();
        assert!(file_path.exists());
    }
}
