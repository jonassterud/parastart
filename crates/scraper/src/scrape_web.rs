//! Scrape takeoffs from flightlog.org

use anyhow::anyhow;
use regex::Regex;
use server_lib::models::NewTakeoff;
use sqlx::PgConnection;
use thirtyfour::{error::WebDriverError, DesiredCapabilities, WebDriver};
use thirtyfour::{By, ChromiumLikeCapabilities, WebElement};
use tracing::info;

/// Scrape a list of takeoffs, as [`NewTakeoff`]'s, and save them to a database, `db`.
pub async fn scrape_takeoffs(db: &mut PgConnection, urls: &[String]) -> Result<(), anyhow::Error> {
    let driver = init_driver().await?;

    for (i, url) in urls.iter().enumerate() {
        info!("Scraping {} / {}", i + 1, urls.len());

        let takeoff = scrape_takeoff(&driver, url).await?;
        sqlx::query!(
            r#"
                INSERT INTO takeoffs(name, description, image, region, altitude, altitude_diff, latitude, longitude, wind_dirs, info_url, source_url, created, updated)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            takeoff.name,
            takeoff.description,
            takeoff.image,
            takeoff.region,
            takeoff.altitude,
            takeoff.altitude_diff,
            takeoff.latitude,
            takeoff.longitude,
            &takeoff.wind_dirs,
            takeoff.info_url,
            takeoff.source_url,
            takeoff.created,
            takeoff.updated,
        )
        .execute(&mut *db)
        .await?;
    }

    Ok(())
}

/// Initialize the web driver.
async fn init_driver() -> Result<WebDriver, WebDriverError> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_no_sandbox()?;
    caps.set_disable_dev_shm_usage()?;

    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.maximize_window().await?;
    driver.goto("https://flightlog.org/").await?;
    sleep(1);

    Ok(driver)
}

/// Scrape a specific takeoff.
#[rustfmt::skip]
async fn scrape_takeoff(driver: &WebDriver, url: &str) -> Result<NewTakeoff, anyhow::Error> {
    sleep(1);
    driver.goto(url).await?;
    sleep(1);
    
    let name = driver.find(By::Css("body > div > table:nth-child(2) > tbody > tr:nth-child(2) > td > table > tbody > tr:nth-child(3) > td > span")).await?;
    let description = driver.find(By::XPath("//td[contains(.,'Description')]/following-sibling::td")).await?;
    let image =  description.find(By::Css("a > img")).await.ok();
    let region = driver.find(By::XPath("//td[contains(.,'region')]/following-sibling::td")).await?;
    let altitude_info = driver.find(By::XPath("//td[contains(.,'Altitude')]/following-sibling::td")).await?;
    let (altitude, altitude_diff) = extract_altitude_info(&altitude_info.text().await?)?;
    let coordinates = driver.find(By::XPath("//td[contains(.,'Coordinates')]/following-sibling::td")).await?;
    let (latitude, longitude) = dms_to_dec(&coordinates.text().await?)?;
    let wind_dirs = description.find(By::Css("img")).await.ok();
    let wind_dirs = if let Some(e) = wind_dirs { e.attr("alt").await?.unwrap_or_default().split(' ').filter(|e| e != &" " && e != &"").map(|e| e.to_owned()).collect() } else { Vec::new() };
    let info_url = driver.find(By::XPath("//td[contains(.,'Link to more info')]/following-sibling::td/a")).await.ok();
    let info_url = if let Some(e) = info_url { e.attr("href").await? } else { None };
    let created = driver.find(By::XPath("//td[contains(.,'created')]/following-sibling::td")).await?;
    let updated = driver.find(By::XPath("//td[contains(.,'Updated')]/following-sibling::td")).await?;

    Ok(NewTakeoff {
        name: name.text().await?,
        description: description.text().await?,
        image: as_png(&driver, image).await?,
        region: region.text().await?,
        altitude: altitude,
        altitude_diff: altitude_diff,
        latitude: latitude,
        longitude: longitude,
        wind_dirs: wind_dirs,
        info_url: info_url,
        source_url: Some(url.to_owned()),
        created: created.text().await?,
        updated: updated.text().await?,
    })
}

/// Sleep for `secs` seconds.
fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}

/// Opens image in a new window and takes a screenshot.
async fn as_png(driver: &WebDriver, element: Option<WebElement>) -> Result<Option<Vec<u8>>, anyhow::Error> {
    if let Some(element) = element {
        let source = element.parent().await?.attr("href").await?.ok_or(anyhow!("missing image source"))?;
        let image = driver.in_new_tab(|| async {
            driver.goto(source).await?;
            let element = driver.find(By::Css("img")).await?;
            
            element.screenshot_as_png().await
        }).await?;

        Ok(Some(image))
    } else {
        Ok(None)
    }
}

/// Convert a string of DMS coordinates to latitude and longitude.
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
