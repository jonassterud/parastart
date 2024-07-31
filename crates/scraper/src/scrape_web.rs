#![deny(missing_docs)]

//! Scrape takeoffs from flightlog.org.

use anyhow::anyhow;
use futures::future::OptionFuture;
use regex::Regex;
use server_lib::helpers;
use server_lib::models::NewTakeoff;
use sqlx::PgConnection;
use thirtyfour::{error::WebDriverError, DesiredCapabilities, WebDriver};
use thirtyfour::{By, ChromiumLikeCapabilities, WebElement};
use tracing::{error, info};

/// Seconds to wait before redirecting to a new URL.
const PAGE_BEFORE_DELAY: u64 = 2;
/// Seconds to wait before starting scraping after redirecting.
const PAGE_SCRAPE_DELAY: u64 = 2;

/// Scrape takeoffs and save them to the database.
///
/// # Arguments
/// 
/// * `urls` - A list of URLs to scrape.
/// * `conn` - A connection to the Postgres database.
/// 
/// # Errors
///
/// This function will return an error if initializing the chrome driver fails.
/// All other errors are logged.
#[rustfmt::skip]
pub async fn try_scrape_all(urls: Vec<String>, conn: &mut PgConnection) -> Result<(), anyhow::Error> {
    let driver = init_driver().await?;

    for (i, url) in urls.iter().enumerate() {
        info!("Scraping {} / {}", i + 1, urls.len());
        try_scrape_and_insert(&url, conn, &driver).await.map_err(|err| error!("{url}: {err}")).ok();
    }

    Ok(())
}

/// Try scraping a takeoff and save to the database.
///
/// # Arguments
/// 
/// * `url` - A URL to scrape.
/// * `conn` - A connection to the Postgres database.
/// * `driver` - A Chrome driver.
/// 
/// # Errors
///
/// This function will return an error if scraping or inserting fails.
#[rustfmt::skip]
async fn try_scrape_and_insert(url: &str, conn: &mut PgConnection, driver: &WebDriver) -> Result<(), anyhow::Error> {
    let takeoff = scrape_takeoff(url, &driver).await?;
    helpers::insert_takeoff(&mut *conn, &takeoff).await?;

    Ok(())
}

/// Scrape a takeoff.
///
/// # Arguments
/// 
/// * `url` - A URL to scrape.
/// * `driver` - A Chrome driver.
/// 
/// # Errors
///
/// This function will return an error if any part of the scraping fails.
/// 
/// # Returns
/// 
/// A [`NewTakeoff`].
#[rustfmt::skip]
async fn scrape_takeoff(url: &str, driver: &WebDriver) -> Result<NewTakeoff, anyhow::Error> {
    sleep(PAGE_BEFORE_DELAY);
    driver.goto(url).await?;
    sleep(PAGE_SCRAPE_DELAY);
    
    let name = driver.find(By::Css("body > div > table:nth-child(2) > tbody > tr:nth-child(2) > td > table > tbody > tr:nth-child(3) > td > span")).await?.text().await?;
    let description = driver.find(By::XPath("//td[contains(.,'Description')]/following-sibling::td")).await?;
    let image = OptionFuture::from(description.find(By::Css("a > img")).await.ok().map(|element| as_png(element, driver))).await.transpose().ok().flatten();
    let region = driver.find(By::XPath("//td[contains(.,'region')]/following-sibling::td")).await?.text().await?;
    let (altitude, altitude_diff) = extract_altitude_info(&driver.find(By::XPath("//td[contains(.,'Altitude')]/following-sibling::td")).await?.text().await?)?;
    let (latitude, longitude) = dms_to_dec(&driver.find(By::XPath("//td[contains(.,'Coordinates')]/following-sibling::td")).await?.text().await?)?;
    let wind_dirs = description.find(By::Css("img")).await.ok();
    let wind_dirs = if let Some(e) = wind_dirs { e.attr("alt").await?.unwrap_or_default().split(' ').filter(|e| e != &" " && e != &"").map(|e| e.to_owned()).collect() } else { Vec::new() };
    let info_url = driver.find(By::XPath("//td[contains(.,'Link to more info')]/following-sibling::td/a")).await.ok();
    let info_url = if let Some(e) = info_url { e.attr("href").await? } else { None };
    let created = driver.find(By::XPath("//td[contains(.,'created')]/following-sibling::td")).await?.text().await?;
    let updated = driver.find(By::XPath("//td[contains(.,'Updated')]/following-sibling::td")).await?.text().await?;
    let description = description.text().await?;
    let source_url = Some(url.to_owned());

    Ok(NewTakeoff {
        name,
        description,
        image,
        region,
        altitude,
        altitude_diff,
        latitude,
        longitude,
        wind_dirs,
        info_url,
        source_url,
        created,
        updated,
    })
}

/// Sleep in the current thread.
/// 
/// # Arguments
/// 
/// * `secs` - Amount of seconds to sleep.
#[rustfmt::skip]
fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}

/// Opens image in a new window and takes a screenshot.
/// 
/// # Arguments
/// 
/// * `element` - The `img` element, where the parent element is an element with the `href` attribute to the source.
/// * `driver` - A Chrome driver.
/// 
/// # Errors
///
/// This function will return an error if it can't find the image source or there's a driver issue.
/// 
/// # Returns
/// 
/// A PNG image in bytes.
#[rustfmt::skip]
async fn as_png(element: WebElement, driver: &WebDriver) -> Result<Vec<u8>, anyhow::Error> {
    let source = element.parent().await?.attr("href").await?.ok_or(anyhow!("missing image source"))?;

    driver.in_new_tab(|| async {
        driver.goto(source).await?;
        driver.find(By::Css("img")).await?.screenshot_as_png().await
    }).await.map_err(|err| anyhow!("{err}"))
}

/// Convert a string of DMS coordinates to latitude and longitude.
///
/// # Arguments
/// 
/// `text` - DMS coordinates (e.g. `DMS: N 60° 38' 44''  E 6° 24' 28''`).
/// 
/// # Errors
///
/// This function will return an error if Regex or parsing fails.
/// 
/// # Returns
/// 
/// A tuple where the first value is the latitude and the second value is the longitude.
#[rustfmt::skip]
fn dms_to_dec(text: &str) -> Result<(f64, f64), anyhow::Error> {
    let re = Regex::new(r"(\d+)")?;
    let mut caps = re.captures_iter(text);

    let nd = caps.next().unwrap().get(1).unwrap().as_str().parse::<f64>()?;
    let nm = caps.next().unwrap().get(1).unwrap().as_str().parse::<f64>()?;
    let ns = caps.next().unwrap().get(1).unwrap().as_str().parse::<f64>()?;
    let ed = caps.next().unwrap().get(1).unwrap().as_str().parse::<f64>()?;
    let em = caps.next().unwrap().get(1).unwrap().as_str().parse::<f64>()?;
    let es = caps.next().unwrap().get(1).unwrap().as_str().parse::<f64>()?;

    let latitude = nd + (nm / 60_f64) + (ns / 3600_f64);
    let longitude = ed + (em / 60_f64) + (es / 3600_f64);

    Ok((latitude, longitude))
}

/// Extract altitude info from a string.
/// 
/// # Arguments
/// 
/// * `text` - Latitude string (e.g. `	790 meters asl Top to bottom 740 meters`).
/// 
/// # Errors
///
/// This function will return an error if Regex or parsing fails.
/// 
/// # Returns
/// 
/// A tuple where the first value is the altitude and the second value is the altitude difference.
#[rustfmt::skip]
fn extract_altitude_info(text: &str) -> Result<(Option<i32>, Option<i32>), anyhow::Error> {
    let re = Regex::new(r"(\d+)")?;
    let mut caps = re.captures_iter(text);

    let altitude = caps.next().map(|e| e.get(1).unwrap().as_str().parse::<i32>()).transpose()?;
    let altitude_diff = caps.next().map(|e| e.get(1).unwrap().as_str().parse::<i32>()).transpose()?;

    Ok((altitude, altitude_diff))
}

/// Initialize and configure the web driver.
///
/// # Errors
///
///  This function will return an error if initialization fails.
///
/// # Returns
///
/// A Chrome driver.
async fn init_driver() -> Result<WebDriver, WebDriverError> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_no_sandbox()?;
    caps.set_disable_dev_shm_usage()?;

    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.maximize_window().await?;

    Ok(driver)
}
