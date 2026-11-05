// the animated toys: `top` and `rm -rf /`. they take over the terminal until
// they finish (the meltdown) or you press q / ctrl+c (top); lib.rs checks
// `is_busy` and sends keys to `stop` while one is running
use std::cell::Cell;

thread_local! {
  static BUSY: Cell<bool> = Cell::new(false);
  static STOP: Cell<bool> = Cell::new(false);
}

pub fn is_busy() -> bool {
  BUSY.with(|b| b.get())
}

/// ask the running animation to stop (top only; the meltdown plays out)
pub fn stop() {
  STOP.with(|s| s.set(true));
}

#[cfg(target_arch = "wasm32")]
mod anim {
  use super::{BUSY, STOP};
  use crate::fun;
  use crate::io::csleep;
  use ansi_term::Colour;
  use rand::prelude::*;
  use xterm_js_rs::Terminal;

  fn begin() {
    BUSY.with(|b| b.set(true));
    STOP.with(|s| s.set(false));
  }

  fn finish(term: &Terminal) {
    BUSY.with(|b| b.set(false));
    crate::term::prompt(term);
    crate::term::follow_cursor();
  }

  fn clock() -> String {
    let now = js_sys::Date::new_0();
    format!("{:02}:{:02}:{:02}", now.get_hours(), now.get_minutes(), now.get_seconds())
  }

  /// `top`: a live process list on the alternate screen (so quitting puts the
  /// terminal back exactly as it was), redrawn every second until q
  pub fn top(term: Terminal) {
    begin();
    wasm_bindgen_futures::spawn_local(async move {
      // alternate screen, hide the cursor
      term.write("\x1b[?1049h\x1b[?25l");
      let mut rng = thread_rng();
      let up = fun::window_json("kfSysinfo").and_then(|i| i.get("uptime_secs")?.as_u64()).unwrap_or(0);
      let users = fun::window_json("kfWho").and_then(|w| w.as_array().map(|a| a.len())).unwrap_or(1).max(1);
      let started = js_sys::Date::now();

      while !STOP.with(|s| s.get()) {
        let elapsed = ((js_sys::Date::now() - started) / 1000.0) as u64;
        let frame = fun::top_frame(&mut rng, &clock(), up + elapsed, users, term.get_cols() as usize);
        // \x1b[K clears each line's tail, so a shorter row doesn't leave bits of the last one
        term.write(&format!("\x1b[H{}\x1b[J", frame.join("\x1b[K\r\n")));
        // a second between frames, but notice q quickly
        for _ in 0..10 {
          if STOP.with(|s| s.get()) {
            break;
          }
          csleep(100).await;
        }
      }

      term.write("\x1b[?25h\x1b[?1049l");
      finish(&term);
    });
  }

  /// `rm -rf /`: everything goes, the kernel panics, then... just kidding
  pub fn meltdown(term: Terminal) {
    begin();
    wasm_bindgen_futures::spawn_local(async move {
      let mut rng = thread_rng();
      let red = |s: &str| Colour::Red.paint(s).to_string();

      for (i, path) in fun::DOOMED.iter().enumerate() {
        term.writeln(&format!("removed '{}'", path));
        // speeds up as it goes, like it's really getting into it
        csleep((90 - i as u64 * 4).max(15)).await;
      }
      for _ in 0..40 {
        let junk: String = (0..rng.gen_range(20, 60)).map(|_| *b"/.-_#%@01xX".choose(&mut rng).unwrap() as char).collect();
        term.writeln(&format!("removed '/{}'", junk));
        csleep(12).await;
      }

      term.writeln("");
      term.writeln(&red("rm: cannot remove '/proc/1': Device or resource busy"));
      csleep(500).await;
      term.writeln(&red("Kernel panic - not syncing: Attempted to kill init! exitcode=0x00000009"));
      term.writeln(&red("---[ end Kernel panic - not syncing: Attempted to kill init! ]---"));
      csleep(2200).await;

      crate::term::clear(&term);
      csleep(900).await;
      for line in ["...", "", "...just kidding.", "", "nothing was deleted. nice try, though."] {
        for c in line.chars() {
          term.write(&c.to_string());
          csleep(45).await;
        }
        term.writeln("");
        csleep(350).await;
      }
      finish(&term);
    });
  }
}

#[cfg(target_arch = "wasm32")]
pub use anim::{meltdown, top};
