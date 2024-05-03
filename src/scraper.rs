use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::{Child, Command};
use std::time::Duration;

use thirtyfour::{By, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};

use crate::consts::*;
use crate::naming::NameConfig;

fn start_chrome_driver() -> Child {
    let child = Command::new("chromedriver/chromedriver")
        .arg("--port=8080")
        .spawn()
        .expect("Failed to start WebDriver server");

    return child;
}

async fn get_web_driver(url: &str) -> WebDriver {
    let mut caps = DesiredCapabilities::chrome();
    // The magic numbers are the width and height of the browser elements, they are not part of the page itself.
    // But only window size can be set, so we have to account for them.
    let width = IMAGE_WIDTH + 14;
    let height = IMAGE_HEIGHT + 145;
    caps.add_arg(format!("--window-size={},{}", width, height))
        .expect("Failed to set window size");
    let driver = WebDriver::new("http://localhost:8080", caps)
        .await
        .expect("Failed to create WebDriver");
    driver.get(url).await.expect("Failed to load page");
    // Let the page load
    tokio::time::sleep(Duration::from_secs(2)).await;

    return driver;
}

async fn delete_element(driver: &WebDriver, by: By) {
    let element = driver.find(by).await.expect("Failed to find element");
    let element_value = element.to_json().expect("Failed to get element value");
    let args = vec![element_value];
    let _ = driver
        .execute("arguments[0].parentNode.removeChild(arguments[0]);", args)
        .await
        .expect("Failed to execute javascript");
}

async fn delete_elements(driver: &WebDriver) {
    // cookies
    delete_element(
        &driver,
        By::Css(".mihoyo-cookie-tips.mihoyo-cookie-tips--bottom.mihoyo-cookie-tips--pc"),
    )
    .await;

    // main gui
    delete_element(&driver, By::Id("frame")).await;
}

async fn get_images(driver: &WebDriver) -> Vec<Vec<u8>> {
    let canvas = driver.find(By::Id("webglCanvas")).await.expect("Failed to find canvas");
    // TODO: still kinda slow
    let mut images_async = Vec::new();
    for _ in RANGE {
        images_async.push(canvas.screenshot_as_png());
    }

    let mut images = Vec::new();
    for image in images_async {
        images.push(image.await.expect("Failed to take screenshot"));
    }

    return images;
}

fn save_images(images_list: &Vec<Vec<u8>>, run_count: u32) {
    create_dir_all(format!("{}/run{}", IMAGES_DIR_PATH, run_count)).expect("Failed to create directory");

    for (i, image) in images_list.iter().enumerate() {
        let path = format!("{}/run{}/image{}.png", IMAGES_DIR_PATH, run_count, i);
        let mut file = File::create(path).expect("Unable to create file");
        file.write_all(image).expect("Unable to write to file");
    }
}

async fn cleanup(driver: WebDriver, mut child: Child) {
    driver.quit().await.expect("Failed to quit WebDriver");
    child.kill().expect("Failed to kill WebDriver server");
}

pub async fn fetch_images(name_config: &mut NameConfig) {
    let child = start_chrome_driver();
    let driver = get_web_driver(URL).await;

    delete_elements(&driver).await;

    let images_list = get_images(&driver).await;
    save_images(&images_list, name_config.scrape_run_count);
    name_config.scrape_run_count += 1;

    cleanup(driver, child).await;
}
