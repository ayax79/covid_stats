mod cache;
mod http;

use cache::Cache;
use dirs;
use http::HttpClient;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cache_dir = cache_dir()?;

    let cache = Cache::new(&cache_dir)?;
    let http_client = HttpClient::new(&cache);

    let path = http_client.fetch_us().await?;
    println!("Fetched US results info to path: {:?}", path);

    Ok(())
}

fn cache_dir() -> std::io::Result<PathBuf> {
    let mut cache_dir = dirs::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No home directory",
    ))?;
    cache_dir.push(".covid_data");

    if !cache_dir.exists() {
        std::fs::create_dir(&cache_dir)?;
    }
    Ok(cache_dir)
}
