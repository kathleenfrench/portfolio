use crate::app::AppConfig;
use crate::io::{csleep, delayed_print, new_line, print};
use yansi::Paint;
use rand::prelude::*;
use colored::*;

use crate::content::{INTRO_LOGS_FULL, INTRO_MSG_FULL};

pub async fn run(cfg: &AppConfig) {
  const SPINNERS: &[&str] = &["/", "-", "\\", "|"];
  const SPINNER_SLEEP: u64 = 50;
  const TEXT_SLEEP: u64 = 15;
  const MAX_SPINNER_LOOPS: u64 = 20;
  const SPINNER_START: u64 = 0;
  const SPINNER_LIMIT: u64 = 150;


  for l in INTRO_MSG_FULL.iter() {
    // let ll = format!("{}\r\n", l.green().bold()).to_string();
    // delayed_print(ll, TEXT_SLEEP).await;
    delayed_print(Paint::yellow(format!("{}\r\n", l)).to_string(), TEXT_SLEEP).await;
    print("").await;
  }

  let mut rng = thread_rng();
  let mut log = "";

  for n in SPINNER_START..SPINNER_LIMIT {
    let spinner_loops = rng.gen_range(1, MAX_SPINNER_LOOPS);

    let last_log = log;
    log = &INTRO_LOGS_FULL.choose(&mut rng).unwrap_or(&"");

    while log == last_log {
      log = &INTRO_LOGS_FULL.choose(&mut rng).unwrap_or(&"");
    }

    let resolution_id = 1 + rng.gen::<u8>() % 100;
    let mut resolution = match resolution_id {
        1..=4 => "FAIL",
        5..=9 => "YES",
        10..=14 => "SUCCESS",
        _ => "OK",
    };

    let mut first = true;

    'outer: for _ in SPINNER_START..spinner_loops {
      for spinner in SPINNERS {
        let msg = format!("{}... {}", log, spinner);

        if first {
          delayed_print(msg, TEXT_SLEEP).await;
          first = false;
        } else {
          print(msg).await;
        }

        csleep(SPINNER_SLEEP).await;
        print("\r").await;

        if cfg.should_quit() {
          resolution = "ABORTED";
          break 'outer;
        }
      }
    }

    let color_func = if resolution == "FAIL" || resolution == "ABORTED" {
        Paint::red
    } else if resolution_id > 50 {
        Paint::white
    } else {
        let color_id = 1 + rng.gen::<u8>() % 20;
        match color_id {
            1..=2 => Paint::red,
            3..=4 => Paint::green,
            5..=6 => Paint::cyan,
            7..=10 => Paint::blue,
            _ => Paint::white,
        }
    };

    print(color_func(format!("{}... {}", log, resolution)).to_string()).await;

    if cfg.should_quit() {
      print("\nALL DONE\n").await;
      return;
    }

    new_line().await;
  }
}