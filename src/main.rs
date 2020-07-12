mod http;
mod cache;

use dirs;
use cache::Cache;
use http::HttpClient;

pub(crate) const US_DATA: &str = "https://raw.githubusercontent.com/nytimes/covid-19-data/master/us.csv";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let home_dir = dirs::home_dir().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No home directory"))?;

    let cache = Cache::new(&home_dir)?;
    let http_client = HttpClient::new();

    let us_csv = cache.us();
    if !us_csv.exists() {
        http_client.fetch(US_DATA, &us_csv).await?;
    }

    Ok(())
}
