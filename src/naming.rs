use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions};

use crate::consts::*;

#[derive(Serialize, Deserialize)]
pub struct NameConfig {
    pub scrape_run_count: u32,
    pub video_count: HashMap<u32, u32>,
}

pub fn read_name_config() -> NameConfig {
    create_dir_all(GEN_DIR_PATH).expect("Unable to create gen directory");

    let default_counts = NameConfig {
        scrape_run_count: 0,
        video_count: HashMap::new(),
    };
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(NAME_CONFIG_PATH)
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
        .truncate(true)
        .create(true)
        .open(NAME_CONFIG_PATH)
        .expect("Unable to open name config file");

    serde_json::to_writer(&file, counts).expect("Unable to write to name config file");
}
