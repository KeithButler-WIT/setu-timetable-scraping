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

    // scrape_all(driver).await?;

    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

// async fn scrape_all(driver: WebDriver) -> Result<(), Box<dyn Error>> {
//     driver
//         .execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
//         .await?;
//     thread::sleep(Duration::from_secs(1));
//
//     let mut wtr = csv::Writer::from_path("setu_timetable.csv")?;
//
//     loop {
//         if let Ok(next_page_button) = driver.find(By::Css("#site-content > div > div.p1szzjq8.dir.dir-ltr > div > div > div > nav > div > a.l1ovpqvx.c1ytbx3a.dir.dir-ltr")).await {
//
//             match next_page_button.is_clickable().await? {
//                 true => {
//
//                     //start extracting data
//
//                     let house_elems = get_house_elements(&driver).await?;
//
//                     for house_elem in house_elems {
//
//                         let bnb_details = BnbDetails::from(house_elem).await?;
//
//                         wtr.serialize(bnb_details)?;
//
//                     }
//                     load_next_page(next_page_button, &driver).await?;
//                 }
//                 false => {
//                     break
//                 },
//             }
//         } else {
//             let house_elems = get_house_elements(&driver).await?;
//
//             for house_elem in house_elems {
//
//                 let bnb_details = BnbDetails::from(house_elem).await?;
//                 wtr.serialize(bnb_details)?;
//             }
//             break;
//         }
//     }
//     Ok(())
// }
//
// async fn load_next_page(
//     next_page_button: WebElement,
//     driver: &WebDriver,
// ) -> Result<(), Box<dyn Error>> {
//
//     next_page_button.click().await?;
//     thread::sleep(Duration::from_secs(2));
//
//     driver
//         .execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
//         .await?;
//     thread::sleep(Duration::from_secs(1));
//
//     Ok(())
// }
//
// async fn get_house_elements(driver: &WebDriver) -> Result<Vec<WebElement>, WebDriverError> {
//     driver.find_all(By::Css("#site-content > div > div:nth-child(2) > div > div > div > div > div.gsgwcjk.g8ge8f1.g14v8520.dir.dir-ltr > div.dir.dir-ltr > div > div.c1l1h97y.dir.dir-ltr > div > div > div > div.cy5jw6o.dir.dir-ltr > div > div.g1qv1ctd.c1v0rf5q.dir.dir-ltr")).await
// }

async fn search_timetable(driver: &WebDriver) -> Result<(), WebDriverError> {
// click_choose_place(driver).await?;
    click_school(driver).await?;
    click_department(driver).await?;
    click_programme(driver).await?;
    click_group(driver).await?;

    // write_place(driver, place).await?;
    //
    // click_search_button(driver).await?;
    // click_generate_timetable(driver).await?;

    Ok(())
}
//
// async fn click_search_button(driver: &WebDriver) -> Result<(), WebDriverError> {
// driver.find(By::Css("#search-tabpanel > div.i1flv5qo.dir.dir-ltr > div.c6ezw63.c1geg2ah.dir.dir-ltr > div.c192dx2b.ckzf1ch.dir.dir-ltr > div.s31emu3.dir.dir-ltr > button")).await?.click().await?;
// Ok(())
// }
//
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

// #[derive(Debug, Serialize)]
// struct BnbDetails {
// title: String,
// description: String,
// host: String,
// availability: String,
// price: String,
// star: String,
// }
//
// impl BnbDetails {
// async fn from(house_elem: WebElement) -> Result<Self, WebDriverError> {
// let title = BnbDetails::get_title(&house_elem).await?;
// let description = BnbDetails::get_description(&house_elem).await?;
// let host = BnbDetails::get_host(&house_elem).await?;
// let availability = BnbDetails::get_availability(&house_elem).await?;
// let price = BnbDetails::get_price(&house_elem).await?;
// let star = BnbDetails::get_star(&house_elem).await?;
//
// Ok(Self {
// title,
// description,
// host,
// availability,
// price,
// star,
// })
// }
// async fn get_title(house_elem: &WebElement) -> Result<String, WebDriverError> {
// house_elem
// .find(By::Css("div:nth-child(1)"))
// .await?
// .text()
// .await
// }
// async fn get_description(house_elem: &WebElement) -> Result<String, WebDriverError> {
// house_elem
// .find(By::Css("div:nth-child(2) > span"))
// .await?
// .text()
// .await
// }
// async fn get_host(house_elem: &WebElement) -> Result<String, WebDriverError> {
// let host = house_elem
// .find(By::Css("div:nth-child(3) > span > span"))
// .await;
// if let Ok(host) = host {
// host.text().await
// } else {
// house_elem
// .find(By::Css("div:nth-child(3) > span"))
// .await?
// .text()
// .await
// }
// }
// async fn get_availability(house_elem: &WebElement) -> Result<String, WebDriverError> {
// house_elem
// .find(By::Css("div:nth-child(4) > span > span"))
// .await?
// .text()
// .await
// }
// async fn get_price(house_elem: &WebElement) -> Result<String, WebDriverError> {
// house_elem
// .find(By::XPath("div[5]/div/div/span[1]/div/span[1]"))
// .await?
// .text()
// .await
// }
// async fn get_star(house_elem: &WebElement) -> Result<String, WebDriverError> {
// if let Ok(star) = house_elem
// .find(By::Css("span > span.r1dxllyb.dir.dir-ltr"))
// .await
// {
// return star.text().await;
// }
// Ok("No ratings available".into())
// }
// }
