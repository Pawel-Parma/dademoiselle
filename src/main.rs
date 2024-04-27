mod consts;
mod merger;
mod naming;
mod scraper;

use thirtyfour::prelude::*;
use tokio;

use crate::merger::*;
use crate::naming::*;
use crate::scraper::*;

#[tokio::main]
async fn main() -> i32 {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide an argument. Use 'fetch' or 'merge'");
        return -1;
    }

    let mut name_config = read_name_config()?;

    if args[1] == "fetch" {
        let url = "https://act.hoyoverse.com/ys/event/e20240419arlecchino-vn0wpz/index.html";
        let range = 0..10;
        fetch_images(url, range, &mut name_config).await?;
    } else if args[1] == "merge" {
        merge_images(0)?;
    } else {
        println!("Invalid argument provided. Use 'fetch' or 'merge'");
    }

    write_name_config(&name_config).expect("Unable to write to file");

    return 0;
}
