use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Range;
use std::process::Child;
use std::process::Command;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio;

fn get_chrome_driver() -> Child {
    // Start the WebDriver server
    let child = Command::new("chromedriver-win64/chromedriver.exe")
        .arg("--port=8080")
        .spawn()
        .expect("Failed to start WebDriver server");

    return child;
}

async fn get_web_driver(url: &str) -> WebDriverResult<WebDriver> {
    // create a WebDriver session
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:8080", caps).await?;

    // Navigate to webpage
    driver.get(url).await?;

    // Let the page load
    tokio::time::sleep(Duration::from_secs(2)).await;

    return Ok(driver);
}

async fn hide_element(driver: &WebDriver, by: By) -> WebDriverResult<()> {
    // Hide the element
    let element = driver.find(by).await?;
    let element_value = element.to_json()?;
    let args = vec![element_value];
    let _ = driver.execute("arguments[0].parentNode.removeChild(arguments[0]);", args).await?;

    return Ok(());
}

async fn hide_elements(driver: &WebDriver) -> WebDriverResult<()> {
    // Hide cookies
    hide_element(&driver, By::Css(".mihoyo-cookie-tips.mihoyo-cookie-tips--bottom.mihoyo-cookie-tips--pc")).await?;

    // Hide main gui
    hide_element(&driver, By::Id("frame")).await?;

    return Ok(());
}

async fn get_images(driver: &WebDriver, range: Range<i32>) -> WebDriverResult<Vec<Vec<u8>>> {
    // prepare a list to store the images
    let mut images_list = Vec::new();

    // Find canvas by ID
    let canvas = driver.find(By::Id("webglCanvas")).await?;

    // Get canvas as PNG
    for i in range {
        let canvas_png = canvas.screenshot_as_png().await?;
        images_list.push(canvas_png);

        print!("\rCaptured {} screenshots", i + 1);
        std::io::stdout().flush().unwrap();
    }

    return Ok(images_list);
}

fn save_images(images_list: &Vec<Vec<u8>>) -> std::io::Result<()> {
    // Create a directory to store the images
    fs::create_dir_all("gen/images")?;

    // Save the images
    for (i, canvas_png) in images_list.iter().enumerate() {
        // TODO: add a folder to store canvas, rename canvas to image, move path top level
        let path = format!("gen/images/canvas{}.png", i);
        let mut file = File::create(path).expect("Unable to create file");
        file.write_all(canvas_png).expect("Unable to write to file");

        print!("\rSaved {} screenshots", i + 1);
        std::io::stdout().flush().unwrap();
    }

    return Ok(());
}

async fn cleanup(driver: WebDriver, mut child: Child) -> WebDriverResult<()> {
    // Close the driver
    driver.quit().await?;

    // Stop the WebDriver server
    child.kill().expect("Failed to kill WebDriver server");

    return Ok(());
}

async fn fetch_data(url: &str, range: Range<i32>) -> WebDriverResult<Vec<Vec<u8>>> {
    // Start the WebDriver server
    let child = get_chrome_driver();

    // Create a new WebDriver session
    let driver = get_web_driver(url).await?;

    // Hide the elements that are blocking the canvas
    hide_elements(&driver).await?;

    // Get the images
    let images_list = get_images(&driver, range).await?;

    // Save the images
    save_images(&images_list).expect("Failed to save images");

    // Cleanup
    cleanup(driver, child).await?;

    return Ok(images_list);
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let url = "https://act.hoyoverse.com/ys/event/e20240419arlecchino-vn0wpz/index.html";
    let range = 0..120;
    _ = fetch_data(url, range).await?;

    return Ok(());
}
