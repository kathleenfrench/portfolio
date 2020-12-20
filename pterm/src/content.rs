static INTRO_LOGS: &str = include_str!("../../static/assets/files/logs.txt");
static INTRO_MSG: &str = include_str!("../../static/assets/files/intro.txt");

lazy_static::lazy_static! {
  pub static ref INTRO_MSG_FULL: Vec<&'static str> = INTRO_MSG.lines().collect();
  pub static ref INTRO_LOGS_FULL: Vec<&'static str> = INTRO_LOGS.lines().collect();
}

