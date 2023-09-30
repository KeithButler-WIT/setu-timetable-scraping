use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{prelude::{WebDriverError, ElementWaitable}, By, DesiredCapabilities, WebDriver, WebElement};
use url::Url;
use serde::Serialize;

pub async fn scrape_setu_timetable() -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver().await?;
    // let url = Url::parse("https://www.airbnb.it/")?;
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
    click_school(driver).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    click_department(driver).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    click_programme(driver).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    click_group(driver).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    // write_place(driver, place).await?;
    //
    // click_search_button(driver).await?;
    click_generate_timetable(driver).await?;

    Ok(())
}
async fn click_school(driver: &WebDriver) -> Result<(), WebDriverError> {
    driver
        // .find(By::Css("body > div:nth-child(8) > div > div > div:nth-child(1) > div > div.cd56ld.cb80sj1.dir.dir-ltr > div.h1ta6hky.dir.dir-ltr > div > div > div > header > div > div.cb994eh.dir.dir-ltr > div.lkm6i7z.lr5v90m.l1rzxhu2.l1kj223i.dir.dir-ltr > div > span.ij8oydg.dir.dir-ltr > button:nth-child(1)"))
        .find(By::Css("#cboSchool > option:nth-child(7)"))
        .await?.click().await?;

    Ok(())
}

async fn click_department(driver: &WebDriver) -> Result<(), WebDriverError> {
    driver
        // .find(By::Css("body > div:nth-child(8) > div > div > div:nth-child(1) > div > div.cd56ld.cb80sj1.dir.dir-ltr > div.h1ta6hky.dir.dir-ltr > div > div > div > header > div > div.cb994eh.dir.dir-ltr > div.lkm6i7z.lr5v90m.l1rzxhu2.l1kj223i.dir.dir-ltr > div > span.ij8oydg.dir.dir-ltr > button:nth-child(1)"))
        .find(By::Css("#CboDept > option:nth-child(3)"))
        .await?.click().await?;

    Ok(())
}

async fn click_programme(driver: &WebDriver) -> Result<(), WebDriverError> {
    driver
        // .find(By::Css("body > div:nth-child(8) > div > div > div:nth-child(1) > div > div.cd56ld.cb80sj1.dir.dir-ltr > div.h1ta6hky.dir.dir-ltr > div > div > div > header > div > div.cb994eh.dir.dir-ltr > div.lkm6i7z.lr5v90m.l1rzxhu2.l1kj223i.dir.dir-ltr > div > span.ij8oydg.dir.dir-ltr > button:nth-child(1)"))
        .find(By::Css("#CboPOS > option:nth-child(42)"))
        .await?.click().await?;

    Ok(())
}

async fn click_group(driver: &WebDriver) -> Result<(), WebDriverError> {
    driver
        // .find(By::Css("body > div:nth-child(8) > div > div > div:nth-child(1) > div > div.cd56ld.cb80sj1.dir.dir-ltr > div.h1ta6hky.dir.dir-ltr > div > div > div > header > div > div.cb994eh.dir.dir-ltr > div.lkm6i7z.lr5v90m.l1rzxhu2.l1kj223i.dir.dir-ltr > div > span.ij8oydg.dir.dir-ltr > button:nth-child(1)"))
        .find(By::Css("#CboStudParentGrp > option:nth-child(3)"))
        .await?.click().await?;

    Ok(())
}

async fn click_generate_timetable(driver: &WebDriver) -> Result<(), WebDriverError> {
    driver.find(By::Css("#BtnRetrieve")).await?.click().await?;
    Ok(())
}

async fn click_print_to_pdf(driver: &WebDriver) -> Result<(), WebDriverError> {
    driver.find(By::Css("#BtnRetrieve")).await?.click().await?;
    Ok(())
}
