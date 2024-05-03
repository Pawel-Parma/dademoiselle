use crate::consts::{FRAMERATE, IMAGES_DIR_PATH, VIDEOS_DIR_PATH};
use std::fs;
use std::process::Command;

pub fn merge_images(which_run: u32) {
    fs::create_dir_all(format!("{}/run{}", VIDEOS_DIR_PATH, which_run)).expect("Unable to create directory");

    Command::new("ffmpeg")
        .arg("-framerate")
        .arg(FRAMERATE.to_string())
        .arg("-i")
        .arg(format!("{}/run{}/image%d.png", IMAGES_DIR_PATH, which_run))
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-f")
        .arg("mp4")
        // TODO: add video count.
        .arg(format!("{}/run{}/video-F{}.mp4", VIDEOS_DIR_PATH, which_run, FRAMERATE))
        .output()
        .expect("Failed to merge images into video");
}
