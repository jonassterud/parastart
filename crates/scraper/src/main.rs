mod parse_kml;
mod scrape_web;

use anyhow::anyhow;
use server_lib::connection;
use tracing::info;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Set up log files
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let log_file_name = format!("scraper_{timestamp}.log");
    let file_appender = tracing_appender::rolling::never("logs", log_file_name);
    let (appender, _guard) = tracing_appender::non_blocking(file_appender);

    // Set up logging
    const DEFAULT_FILTER: &str = "scraper=debug";
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_FILTER.into()))
        .with(tracing_subscriber::fmt::layer())
        .with(fmt::layer().pretty().with_writer(std::io::stdout))
        .with(fmt::layer().with_ansi(false).with_writer(appender))
        .init();

    // Connect to database
    let mut connection = connection::single().await.map_err(|e| anyhow!(e))?;

    // Gather URLs
    let path = "crates/scraper/resources/country_160.kml";
    let urls = parse_kml::get_missing_urls(path, &mut connection).await?;

    // Scrape URLs and insert into database
    scrape_web::try_scrape_all(urls, &mut connection).await?;
    info!("Exiting.");
    
    Ok(())
}
