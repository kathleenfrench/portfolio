use crate::app::AppConfig;
use crate::io::{csleep, delayed_print, new_line, print, clear_line};
use yansi::Paint;
use rand::prelude::*;

use crate::content::{INTRO_LOGS_FULL, INTRO_MSG_FULL};

pub async fn run(_cfg: &AppConfig) {
  const LINE_SLEEP: u64 = 1;
  const TEXT_SLEEP: u64 = 15;
  const FAST_TEXT_SLEEP: u64 = 0.75 as u64;
  const MAX_SPINNER_LOOPS: u64 = 10;
  const INTRO_LOG_START: u64 = 0;
  const INTRO_LOG_LIMIT: u64 = 150;
  
  let faux_log_len = INTRO_LOGS_FULL.len() as u64;

  for l in INTRO_MSG_FULL.iter() {
    delayed_print(Paint::yellow(format!("{}\r\n", l)).bold().to_string(), TEXT_SLEEP).await;
    new_line().await;
  }

  clear_line().await;

  let mut rng = thread_rng();
  let mut log = "";

  for n in INTRO_LOG_START..INTRO_LOG_LIMIT {
    let spinner_loops = rng.gen_range(1, MAX_SPINNER_LOOPS);

    let last_log = log;
    log = &INTRO_LOGS_FULL.choose(&mut rng).unwrap_or(&"");

    while log == last_log {
      log = &INTRO_LOGS_FULL.choose(&mut rng).unwrap_or(&"");
    }

    match n {
      0 => {
        delayed_print(Paint::green(format!("{}\r\n", "initializing....")).bold().to_string(), TEXT_SLEEP).await;
      },
      20 => { 
        clear_line().await;
        delayed_print(Paint::green(format!("{}\r\n", "powering up the server farm....")).bold().to_string(), TEXT_SLEEP).await;
      },
      45 => {
        clear_line().await;
        delayed_print(Paint::green(format!("{}\r\n", "provisioning a fleet of 32 core instances...")).bold().to_string(), TEXT_SLEEP).await;
      },
      65 => {
        clear_line().await;
        delayed_print(Paint::green(format!("{}\r\n", "...btw pls sponsor my fleet of 32 core instances")).bold().to_string(), TEXT_SLEEP).await;
      },
      90 => {
        clear_line().await;
        delayed_print(Paint::green(format!("{}\r\n", "burying treasure...maybe you can find it?")).bold().to_string(), TEXT_SLEEP).await
      },
      115 => {
        clear_line().await;
        delayed_print(Paint::green(format!("{}\r\n", "juuuuuust one more second....")).bold().to_string(), TEXT_SLEEP).await;
      },
      _ => println!("n: {}", n),
    }

    'outer: for _ in INTRO_LOG_START..spinner_loops {
      let msg = format!("{}", log);

      print(msg).await;
      csleep(LINE_SLEEP).await;
      print("\r").await;

      if n >= faux_log_len {
        break 'outer;
      }
    }

    delayed_print(Paint::white(format!("{}", log)).to_string(), FAST_TEXT_SLEEP).await;

    if n >= faux_log_len {
      print(Paint::green(format!("\r\n{}\r\n", "DONE")).bold().to_string()).await;
      return;
    }

    new_line().await;
  }

  clear_line().await;
}