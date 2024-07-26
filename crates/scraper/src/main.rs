mod parse_kml;
mod scrape_web;

use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let path = "crates/scraper/resources/country_160.kml";
    let urls = parse_kml::get_urls(path).await?;
    let takeoffs = scrape_web::scrape_takeoffs(&urls).await?;

    println!("{:?}", takeoffs);

    Ok(())
}
