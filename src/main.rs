mod consts;
mod merger;
mod naming;
mod scraper;

use tokio;

use crate::merger::*;
use crate::naming::*;
use crate::scraper::*;

fn check_args_correctness(args: &Vec<String>, name_config: &NameConfig) {
    if args.len() < 2 {
        println!("Please provide an argument. Use 'fetch' or 'merge'");
        std::process::exit(-1);
    }

    if args[1] != "fetch" && args[1] != "merge" {
        println!("Invalid argument provided. Use 'fetch' or 'merge'");
        std::process::exit(-2);
    }

    if args[1] == "fetch" && args.len() != 2 {
        println!("Too many arguments provided. Fetch does not require any arguments");
        std::process::exit(-3);
    }

    if args[1] == "merge" && args.len() < 3 {
        println!("Please provide a run number to merge");
        std::process::exit(-4);
    }
    if args[1] == "merge" && args.len() > 3 {
        println!("Too many arguments provided. Merge requires only one argument");
        std::process::exit(-5);
    }

    if args[1] == "merge" {
        match args[2].parse::<u32>() {
            Ok(parsed) => {
                if parsed >= name_config.scrape_run_count {
                    println!("Run number provided is greater than the run count");
                    std::process::exit(-4);
                }
            }
            Err(_) => {
                println!("Could not parse string to unsigned integer");
                std::process::exit(-5);
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut name_config = read_name_config();
    check_args_correctness(&args, &name_config);

    if args[1] == "fetch" {
        println!("Starting to fetch images...");

        fetch_images(&mut name_config).await;
        println!("Finished fetching images.");
    } else if args[1] == "merge" {
        println!("Starting to merge images...");
        merge_images(args[2].parse::<u32>().unwrap(), &mut name_config);
        println!("Finished merging images.");
    } else {
        println!("Invalid argument provided. Use 'fetch' or 'merge'");
    }

    write_name_config(&name_config);
    println!("Configuration saved.");

    return Ok(());
}
