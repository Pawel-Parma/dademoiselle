use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

use crate::consts::*;

#[derive(Serialize, Deserialize)]
pub struct NameConfig {
    pub scrape_run_count: u32,
}

pub fn read_name_config() -> NameConfig {
    let file_path = NAME_CONFIG_PATH;
    let default_counts = NameConfig { scrape_run_count: 0 };

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open name config file");

    let counts = serde_json::from_reader(&file);

    return match counts {
        Ok(c) => c,
        Err(_) => {
            serde_json::to_writer(&mut file, &default_counts).expect("Unable to write to file");
            default_counts
        }
    };
}

pub fn write_name_config(counts: &NameConfig) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(NAME_CONFIG_PATH)
        .expect("Unable to open name config file");

    serde_json::to_writer(&file, counts).expect("Unable to write to name config file");
}
