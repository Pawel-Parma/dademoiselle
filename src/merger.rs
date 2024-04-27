use crate::consts::{IMAGES_DIR_PATH, VIDEOS_DIR_PATH};
use std::fs;
use std::process::Command;

pub fn merge_images(which_run: u32) {
    fs::create_dir_all(format!("{}/run{}", VIDEOS_DIR_PATH, which_run)).expect("Unable to create directory");

    Command::new("ffmpeg")
        .arg("-framerate")
        .arg("30")
        .arg("-i")
        .arg(format!("{}/run{}/image%d.png", IMAGES_DIR_PATH, which_run))
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        // TODO: add video count.
        // TODO: add option to which run merge
        // TODO: Find why win11 doesnt recognize .mp4 as a video file and only works via ffplay
        .arg(format!("{}/run{}/video.mp4", VIDEOS_DIR_PATH, which_run))
        .output()
        .expect("Failed to merge images into video");
}
