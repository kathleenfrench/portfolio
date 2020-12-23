static INTRO_LOGS: &str = include_str!("../../static/assets/files/logs.txt");
static INTRO_MSG: &str = include_str!("../../static/assets/files/intro.txt");
static RESUME_CONTENT: &str = include_str!("../../static/assets/files/resume.txt");
static RANDOM_STRINGS: &str = include_str!("../../static/assets/files/files.txt");
static TMP_SOURCES: &str = include_str!("../../static/assets/files/sources.txt");
static GEORGE_PICS_FILENAMES: &str = include_str!("../../static/assets/files/george.txt");

lazy_static::lazy_static! {
  pub static ref INTRO_MSG_FULL: Vec<&'static str> = INTRO_MSG.lines().collect();
  pub static ref INTRO_LOGS_FULL: Vec<&'static str> = INTRO_LOGS.lines().collect();
  pub static ref RESUME: Vec<&'static str> = RESUME_CONTENT.lines().collect();
  pub static ref RANDOM_STRINGS_LIST: Vec<&'static str> = RANDOM_STRINGS.lines().collect();
  pub static ref TMP_SOURCE_LIST: Vec<&'static str> = TMP_SOURCES.lines().collect();
  pub static ref GEORGE_PICS: Vec<&'static str> = GEORGE_PICS_FILENAMES.lines().collect();
}

pub static TMP_EXTENSIONS_LIST: &[&str] = &[
  "txt~", "sh~", "py~", "pdf~", "md~", "yaml~",
];

pub static EXTENSIONS_LIST: &[&str] = &[
  "gif", "webm", "mp4", "html", "php", "md", "png", "jpg", "ogg", "mp3", "flac", "iso", "zip",
  "rar", "tar.gz", "tar.bz2", "tar.xz", "deb", "rpm", "exe",
];