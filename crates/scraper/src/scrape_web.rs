//! Scrape takeoffs from flightlog.org

use thirtyfour::{error::WebDriverError, DesiredCapabilities, WebDriver};
use thirtyfour::{By, ChromiumLikeCapabilities};
use server_lib::models::NewTakeoff;

pub async fn scrape_takeoffs(urls: &[String]) -> Result<Vec<NewTakeoff>, WebDriverError> {
    let mut out = Vec::new();
    let driver = init_driver().await?;
    sleep(1);

    for url in urls {
        let takeoff = scrape_takeoff(&driver, url).await?;
        out.push(takeoff);
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

async fn scrape_takeoff(driver: &WebDriver, url: &str) -> Result<NewTakeoff, WebDriverError> {
    driver.goto(url).await?;
    sleep(1);

    todo!()
}

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
