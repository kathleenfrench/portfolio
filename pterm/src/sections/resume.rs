use crate::app::AppConfig;
use crate::io::{csleep, delayed_print, new_line, print, clear_line};
use yansi::Paint;

pub async fn run(cfg: &AppConfig) {
  print(Paint::white(format!("{}", "RESUME")).to_string()).await;
  clear_line().await;
}