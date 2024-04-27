use std::fs::{create_dir_all, File};
use std::io::Write;
use std::ops::Range;
use std::process::{Child, Command};
use std::time::Duration;

use thirtyfour::error::WebDriverResult;
use thirtyfour::{By, DesiredCapabilities, WebDriver};

use crate::consts::*;
use crate::naming::NameConfig;

fn start_chrome_driver() -> Child {
    let child = Command::new("chromedriver/chromedriver.exe")
        .arg("--port=8080")
        .spawn()
        .expect("Failed to start WebDriver server");

    return child;
}

async fn get_web_driver(url: &str) -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:8080", caps).await?;
    driver.get(url).await?;
    // Let the page load
    tokio::time::sleep(Duration::from_secs(2)).await;

    return Ok(driver);
}

async fn delete_element(driver: &WebDriver, by: By) -> WebDriverResult<()> {
    let element = driver.find(by).await?;
    let element_value = element.to_json()?;
    let args = vec![element_value];
    let _ = driver
        .execute("arguments[0].parentNode.removeChild(arguments[0]);", args)
        .await?;

    return Ok(());
}

async fn delete_elements(driver: &WebDriver) -> WebDriverResult<()> {
    // cookies
    delete_element(
        &driver,
        By::Css(".mihoyo-cookie-tips.mihoyo-cookie-tips--bottom.mihoyo-cookie-tips--pc"),
    )
    .await?;

    // main gui
    delete_element(&driver, By::Id("frame")).await?;

    return Ok(());
}

async fn get_images(driver: &WebDriver, range: Range<i32>) -> WebDriverResult<Vec<Vec<u8>>> {
    let canvas = driver.find(By::Id("webglCanvas")).await?;

    let mut images_list = Vec::new();
    for i in range {
        let image = canvas.screenshot_as_png().await?;
        images_list.push(image);

        print!("\rCaptured {} screenshots", i + 1);
        std::io::stdout().flush().unwrap();
    }
    println!();

    return Ok(images_list);
}

fn save_images(images_list: &Vec<Vec<u8>>, run_count: u32) -> std::io::Result<()> {
    create_dir_all(format!("{}/run{}", IMAGES_DIR_PATH, run_count))?;

    for (i, image) in images_list.iter().enumerate() {
        let path = format!("{}/run{}/image{}.png", IMAGES_DIR_PATH, run_count, i);
        let mut file = File::create(path).expect("Unable to create file");
        file.write_all(image).expect("Unable to write to file");

        print!("\rSaved {} screenshots", i + 1);
        std::io::stdout().flush().unwrap();
    }
    println!();

    return Ok(());
}

async fn cleanup(driver: WebDriver, mut child: Child) -> WebDriverResult<()> {
    driver.quit().await?;
    child.kill().expect("Failed to kill WebDriver server");

    return Ok(());
}

pub async fn fetch_images(
    url: &str,
    range: Range<i32>,
    name_config: &mut NameConfig,
) -> WebDriverResult<()> {
    let child = start_chrome_driver();
    let driver = get_web_driver(url).await?;

    delete_elements(&driver).await?;

    let images_list = get_images(&driver, range).await?;
    save_images(&images_list, name_config.scrape_run_count).expect("Failed to save images");
    name_config.scrape_run_count += 1;

    cleanup(driver, child).await?;

    return Ok(());
}
