mod parse_kml;
mod scrape_web;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Set up tracing subscriber
    const DEFAULT_FILTER: &str = "scraper=debug";
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_FILTER.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Gather URLs
    let path = "crates/scraper/resources/country_160.kml";
    let urls = parse_kml::get_urls(path).await?;

    // Scrape URLs
    let takeoffs = scrape_web::scrape_takeoffs(&urls).await?;

    Ok(())
}
