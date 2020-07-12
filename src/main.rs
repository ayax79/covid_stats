mod http;
mod cache;

pub(crate) const US_DATA: &str = "https://raw.githubusercontent.com/nytimes/covid-19-data/master/us.csv";

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
