use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{prelude::{WebDriverError, ElementWaitable}, By, DesiredCapabilities, WebDriver, WebElement};
use url::Url;
use serde::Serialize;

pub async fn scrape_setu_timetable() -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver().await?;
    let url = Url::parse("https://studentssp.wit.ie/Timetables/StudentGroupTT.aspx")?;

    print!("{}", url);

    driver.goto(url).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    search_timetable(&driver).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    tokio::time::sleep(Duration::from_secs(360)).await;

    // scrape_all(driver).await?;

    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

async fn search_timetable(driver: &WebDriver) -> Result<(), WebDriverError> {
// click_choose_place(driver).await?;
    // TODO Create config file
    // TODO check config file for data
    // TODO Config file
    click_css(driver, "#cboSchool > option:nth-child(7)").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    click_css(driver, "#CboDept > option:nth-child(3)").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    click_css(driver, "#CboPOS > option:nth-child(42)").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    click_css(driver, "#CboStudParentGrp > option:nth-child(3)").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    // write_place(driver, place).await?;
    //
    // click_search_button(driver).await?;
    click_css(driver, "#BtnRetrieve").await?;   // Generate Timetable
    click_css(driver, "#btnPrint").await?;      // Print To Pdf

    Ok(())
}

async fn click_css(driver: &WebDriver, css_location: &str) -> Result<(), WebDriverError> {
    driver
        // .find(By::Css("body > div:nth-child(8) > div > div > div:nth-child(1) > div > div.cd56ld.cb80sj1.dir.dir-ltr > div.h1ta6hky.dir.dir-ltr > div > div > div > header > div > div.cb994eh.dir.dir-ltr > div.lkm6i7z.lr5v90m.l1rzxhu2.l1kj223i.dir.dir-ltr > div > span.ij8oydg.dir.dir-ltr > button:nth-child(1)"))
        .find(By::Css(&*css_location))
        .await?.click().await?;

    Ok(())
}
