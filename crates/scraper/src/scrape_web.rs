//! Scrape takeoffs from flightlog.org

use anyhow::anyhow;
use regex::Regex;
use server_lib::models::NewTakeoff;
use thirtyfour::{error::WebDriverError, DesiredCapabilities, WebDriver};
use thirtyfour::{By, ChromiumLikeCapabilities, WebElement};

pub async fn scrape_takeoffs(urls: &[String]) -> Result<Vec<NewTakeoff>, anyhow::Error> {
    let mut out = Vec::new();
    let driver = init_driver().await?;
    sleep(1);

    for url in urls {
        println!("{:?}", url);
        let takeoff = scrape_takeoff(&driver, url).await?;
        out.push(takeoff);

        break; // TODO: TEMP
    }

    Ok(out)
}

async fn init_driver() -> Result<WebDriver, WebDriverError> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_no_sandbox()?;
    caps.set_disable_dev_shm_usage()?;

    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.maximize_window().await?;
    driver.goto("https://flightlog.org/").await?;

    Ok(driver)
}

#[rustfmt::skip]
async fn scrape_takeoff(driver: &WebDriver, url: &str) -> Result<NewTakeoff, anyhow::Error> {
    driver.goto(url).await?;
    sleep(1);
    
    // TODO: Actually check that "holfy_url" is a Holfuy URL
    let name = driver.find(By::Css("body > div > table:nth-child(2) > tbody > tr:nth-child(2) > td > table > tbody > tr:nth-child(3) > td > span")).await?;
    let description = driver.find(By::XPath("//td[contains(.,'Description')]/following-sibling::td")).await?;
    let image =  description.find(By::Css("a > img")).await.ok();
    let region = driver.find(By::XPath("//td[contains(.,'region')]/following-sibling::td")).await?;
    let holfuy_url = driver.find(By::XPath("//td[contains(.,'Link to more info')]/following-sibling::td")).await?;
    let coordinates = driver.find(By::XPath("//td[contains(.,'Coordinates')]/following-sibling::td")).await?;
    let wind_directions = description.find(By::Css("img")).await.ok();
    let wind_directions = if let Some(e) = wind_directions { e.attr("alt").await?.unwrap_or_default().split(' ').map(|e| e.to_owned()).collect() } else { Vec::new() };
    let (latitude, longitude) = dms_to_dec(&coordinates.text().await?)?;

    Ok(NewTakeoff {
        name: name.inner_html().await?,
        description: description.text().await?,
        image: as_png(image).await?,
        region: region.inner_html().await?,
        latitude: latitude,
        longitude: longitude,
        holfuy_url: holfuy_url.attr("href").await?,
        wind_directions: wind_directions,
    })
}

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}

async fn as_png(element: Option<WebElement>) -> Result<Option<Vec<u8>>, WebDriverError> {
    if let Some(element) = element {
        Ok(Some(element.screenshot_as_png().await?))
    } else {
        Ok(None)
    }
}

/// Convert a string of DMS coordinates to latitude and longitude.
///
/// # Panics
///
/// Panics if Regex matching fails.
///
/// # Errors
///
/// This function will return an error if parsing fails.
/// 
/// # Returns
/// 
/// A tuple where the first element is the latitude and the second element is the longitude.
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
