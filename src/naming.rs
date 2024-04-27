use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io;

use crate::consts::*;

#[derive(Serialize, Deserialize)]
pub struct NameConfig {
    pub scrape_run_count: u32,
}

pub fn read_name_config() -> Result<NameConfig, serde_json::Error> {
    let file_path = NAME_CONFIG_PATH;
    let default_counts = NameConfig {
        scrape_run_count: 0,
    };

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let counts = serde_json::from_reader(&file);

    return match counts {
        Ok(c) => Ok(c),
        Err(_) => {
            serde_json::to_writer(&mut file, &default_counts).expect("Unable to write to file");
            Ok(default_counts)
        }
    }
}

pub fn write_name_config(counts: &NameConfig) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(NAME_CONFIG_PATH)?;

    serde_json::to_writer(&file, counts)?;

    return Ok(());
}
