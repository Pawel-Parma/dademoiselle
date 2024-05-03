use crate::consts::{FRAMERATE, IMAGES_DIR_PATH, VIDEOS_DIR_PATH};
use crate::naming::NameConfig;
use std::fs;
use std::process::Command;

pub fn merge_images(which_run: u32, name_config: &mut NameConfig) {
    fs::create_dir_all(format!("{}/run{}", VIDEOS_DIR_PATH, which_run)).expect("Unable to create directory");

    let video = name_config.video_count.entry(which_run).or_insert(0);
    println!("Merging images into video...");
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
        .arg(format!(
            "{}/run{}/video-{}-F{}.mp4",
            VIDEOS_DIR_PATH, which_run, video, FRAMERATE
        ))
        .output()
        .expect("Failed to merge images into video");

    println!("Video created.");
    *video += 1;
}
