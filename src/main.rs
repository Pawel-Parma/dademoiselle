mod consts;
mod merger;
mod naming;
mod scraper;

use tokio;

use crate::merger::*;
use crate::naming::*;
use crate::scraper::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide an argument. Use 'fetch' or 'merge'");
        std::process::exit(1);
    }

    let mut name_config = read_name_config();

    if args[1] == "fetch" {
        fetch_images(&mut name_config).await;
    } else if args[1] == "merge" {
        merge_images(0);
    } else {
        println!("Invalid argument provided. Use 'fetch' or 'merge'");
    }

    write_name_config(&name_config);

    return Ok(());
}
