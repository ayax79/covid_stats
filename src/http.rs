use reqwest::Client;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};

pub struct HttpClient {
    client: Client,
}

impl HttpClient {

    pub fn new() -> Self {
        HttpClient {
            client: Client::new(), 
        }
    }

    pub async fn fetch(&self, url: &str, output: &Path) -> io::Result<()>{
        let mut rs = self.client.get(url)
            .send()
            .await
            .map_err(as_ioerror)?;

        let file = File::create(output)?;        
        let mut wtr = BufWriter::new(file);

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
    use crate::US_DATA;
    use tempfile;

    #[tokio::test]
    async fn test_fetch_us() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let client = HttpClient::new();
        let file_path = tmp_dir.path().join("us.csv");
        assert!(!file_path.exists());
        client.fetch( US_DATA, &file_path).await.unwrap();
        assert!(file_path.exists());
    }

}