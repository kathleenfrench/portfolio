use crate::app::AppConfig;
use crate::io::{csleep, delayed_print, get_terminal_rows, new_line, print, clear_line};
use yansi::Paint;
use rand::prelude::*;

use crate::content::{INTRO_LOGS_FULL, INTRO_MSG_FULL};

// each one clears the screen and starts a new screenful of logs
const MILESTONES: &[&str] = &[
  "initializing....",
  "powering up the server farm....",
  "provisioning a fleet of 32 core instances...",
  "...btw pls sponsor my fleet of 32 core instances",
  "burying treasure...maybe you can find it?",
  "juuuuuust one more second....",
];

pub async fn run(_cfg: &AppConfig) {
  const LINE_SLEEP: u64 = 1;
  const TEXT_SLEEP: u64 = 15;
  // each log line flickers 1-3 times before it lands; higher = slower intro
  const MAX_SPINNER_LOOPS: u64 = 4;

  for l in INTRO_MSG_FULL.iter() {
    delayed_print(Paint::yellow(format!("{}\r\n", l)).bold().to_string(), TEXT_SLEEP).await;
    new_line().await;
  }

  let mut rng = thread_rng();

  for milestone in MILESTONES {
    clear_line().await;
    delayed_print(Paint::green(format!("{}\r\n", milestone)).bold().to_string(), TEXT_SLEEP).await;

    // fill the rest of the screen (everything under the milestone line)
    let lines = random_logs(get_terminal_rows().saturating_sub(1).max(1), &mut rng);
    let last = lines.len() - 1;

    for (i, log) in lines.into_iter().enumerate() {
      // flicker the line a few times before it "lands"
      for _ in 0..rng.gen_range(1, MAX_SPINNER_LOOPS) {
        print(log).await;
        csleep(LINE_SLEEP).await;
        print("\r").await;
      }

      print(Paint::white(log).to_string()).await;

      // no newline after the bottom row, or the screen scrolls
      if i < last {
        new_line().await;
      }
    }
  }

  clear_line().await;
}

/// `count` log lines in random order: a fresh shuffle of every log each pass,
/// so a screenful has no repeats unless it's taller than the whole log file,
/// and never the same line twice in a row
fn random_logs<R: Rng>(count: usize, rng: &mut R) -> Vec<&'static str> {
  let mut out: Vec<&'static str> = Vec::with_capacity(count);

  while out.len() < count {
    let mut pass: Vec<&'static str> = INTRO_LOGS_FULL.iter().copied().filter(|l| !l.trim().is_empty()).collect();
    if pass.is_empty() {
      break;
    }
    pass.shuffle(rng);
    if pass.len() > 1 && out.last() == pass.first() {
      pass.swap(0, 1);
    }
    out.extend(pass);
  }

  out.truncate(count);
  out
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn random_logs_fills_the_screen_without_back_to_back_repeats() {
    let mut rng = thread_rng();
    for count in [1, 40, 500] {
      let lines = random_logs(count, &mut rng);
      assert_eq!(lines.len(), count);
      assert!(lines.windows(2).all(|w| w[0] != w[1]));
    }
  }
}
