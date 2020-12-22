static INTRO_LOGS: &str = include_str!("../../static/assets/files/logs.txt");
static INTRO_MSG: &str = include_str!("../../static/assets/files/intro.txt");
static RESUME_CONTENT: &str = include_str!("../../static/assets/files/resume.txt");
static RANDOM_FILES_LIST: &str = include_str!("../../static/assets/files/files.txt");

lazy_static::lazy_static! {
  pub static ref INTRO_MSG_FULL: Vec<&'static str> = INTRO_MSG.lines().collect();
  pub static ref INTRO_LOGS_FULL: Vec<&'static str> = INTRO_LOGS.lines().collect();
  pub static ref RESUME: Vec<&'static str> = RESUME_CONTENT.lines().collect();
  pub static ref FILES_LIST: Vec<&'static str> = RANDOM_FILES_LIST.lines().collect();
}

pub static EXTENSIONS_LIST: &[&str] = &[
    "gif", "webm", "mp4", "html", "php", "md", "png", "jpg", "ogg", "mp3", "flac", "iso", "zip",
    "rar", "tar.gz", "tar.bz2", "tar.xz", "deb", "rpm", "exe",
];