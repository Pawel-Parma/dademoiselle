use std::ops::Range;

// paths
pub const NAME_CONFIG_PATH: &str = "gen/name_config.json";
pub const GEN_DIR_PATH: &str = "gen";
pub const IMAGES_DIR_PATH: &str = "gen/images";
pub const VIDEOS_DIR_PATH: &str = "gen/videos";

// scraper
pub const URL: &str = "https://act.hoyoverse.com/ys/event/e20240419arlecchino-vn0wpz/index.html";
pub const RANGE: Range<i32> = 0..150;
pub const IMAGE_WIDTH: i32 = 960; // 1920 / 2
pub const IMAGE_HEIGHT: i32 = 540; // 1080 / 2

// merger
pub const FRAMERATE: u32 = 10; // between 8 and 12 is good. Maybe 6 but it's a bit slow
