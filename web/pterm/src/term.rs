use xterm_js_rs::Terminal;
use ansi_term::{Colour, Style};
use rand::thread_rng;
use rand::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;

use crate::content;
use crate::md;
use crate::utils;

pub const KEY_ENTER: u32 = 13;
pub const KEY_BACKSPACE: u32 = 8;
pub const KEY_LEFT_ARROW: u32 = 37;
pub const KEY_RIGHT_ARROW: u32 = 39;
pub const KEY_UP_ARROW: u32 = 38;
pub const KEY_DOWN_ARROW: u32 = 40;
pub const KEY_HOME: u32 = 36;
pub const KEY_END: u32 = 35;
pub const KEY_C: u32 = 67;
pub const KEY_L: u32 = 76;
pub const KEY_U: u32 = 85;

pub const CURSOR_LEFT: &str = "\x1b[D";
pub const CURSOR_RIGHT: &str = "\x1b[C";

pub const VISIBLE_CLASS: &str = "visible";
pub const HIDDEN: &str = "hidden";




/// scroll the page just enough to keep the cursor on screen (with a couple of
/// lines of room under it, for the `? for help` hint). the terminal can
/// be pushed down the page by a photo or the about section above it, and
/// output would otherwise land below the fold. waits a frame for xterm to
/// move its (hidden) input textarea, which tracks the cursor
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "export function follow_cursor() { requestAnimationFrame(() => requestAnimationFrame(() => { const t = document.querySelector('#terminal .xterm-helper-textarea'); if (!t) return; const r = t.getBoundingClientRect(); const room = 3 * r.height; if (r.bottom + room > innerHeight) window.scrollBy(0, r.bottom + room - innerHeight); else if (r.top < 0) window.scrollBy(0, r.top - room); })); }")]
extern "C" {
  pub fn follow_cursor();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn follow_cursor() {}

pub fn prompt(term: &Terminal) {
  // `user@portfolio dir $`, like zsh. once you've become kathleen
  // (`su kathleen`), the name and colour say so
  let (name, colour) = match crate::shell::user() {
    "kathleen" => ("kathleen", Colour::Green),
    _ => ("kathleenfrench", Colour::Yellow),
  };
  let dir = crate::shell::prompt_dir();
  let width = format!("{}@portfolio {} $ ", name, dir).chars().count();

  term.writeln("");
  term.write(&format!(
    "{} {} {} ",
    colour.bold().paint(format!("{}@portfolio", name)),
    Colour::Fixed(110).bold().paint(dir),
    colour.bold().paint("$"),
  ));
  // tui-style hint on the line under the prompt: write it, then move back up
  // to the end of the prompt (relative moves, so it works even if writing the
  // hint scrolled the screen). lib.rs erases it when enter is pressed
  term.write(&format!("\r\n{}\x1b[1A\x1b[{}G", md::DIM.paint(PROMPT_HINT), width + 1));
}

pub const PROMPT_HINT: &str = "? for help";

/// erase the hint under the prompt (everything below the cursor)
pub fn clear_hint(term: &Terminal) {
  term.write("\x1b[J");
}


///// commands
pub fn help_text(term: &Terminal) {
  reset_window(&term);
  new_line(&term);
  commands(&term);
}

/// what you see on page load: a login greeting above the command list
pub fn welcome(term: &Terminal) {
  reset_window(&term);
  new_line(&term);
  md::note(&term, &format!("last login: {} from somewhere on the internet", login_time()));
  new_line(&term);
  commands(&term);
}

// like a real terminal's: `wed nov 4 21:19:28`
fn login_time() -> String {
  let now = js_sys::Date::new_0();
  let days = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
  let months = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
  format!(
    "{} {} {} {:02}:{:02}:{:02}",
    days[now.get_day() as usize], months[now.get_month() as usize], now.get_date(),
    now.get_hours(), now.get_minutes(), now.get_seconds(),
  )
}

fn commands(term: &Terminal) {
  md::heading(&term, "commands");
  md::item(&term, "about", 12, "learn more about me");
  md::item(&term, "resume", 12, "view available subcommands");
  md::item(&term, "george", 12, "show a random picture of george");
  md::item(&term, "henry", 12, "show a random picture of henry");
  md::item(&term, "trout", 12, "show a random picture of trout");
  md::item(&term, "contact", 12, "contact me");
  md::item(&term, "clear", 12, "clear the terminal window");
  md::item(&term, "replay", 12, "replay the intro animation");
  md::item(&term, "hint", 12, "a nudge, if you're looking for something");
  new_line(&term);
  md::note(&term, "type a command and hit enter");
}

pub fn contact_info(term: &Terminal) {
  reset_window(&term);
  new_line(&term);
  md::heading(&term, "contact");
  md::bullet(&term, "github");
  md::bullet(&term, "linkedin");
  md::bullet(&term, "email");
  new_line(&term);
  md::note(&term, "run `goto <link>` to open one, e.g. `goto github`");
}

pub fn goto_links(term: &Terminal, line: &str) {
  let line_split = line.split_ascii_whitespace().collect::<Vec<_>>();
  let target = <&str>::clone(&line_split[1]);

  match target {
      "github" => {
          md::writeln(&term, &format!("redirecting to {}...", target));
          utils::open_in_new_tab("https://github.com/kathleenfrench");
      },
      "linkedin" => {
          md::writeln(&term, &format!("redirecting to {}...", target));
          utils::open_in_new_tab("https://www.linkedin.com/in/frenchkathleen/");
      },
      "email" => {
          utils::open_in_new_tab("https://mail.google.com/mail/?view=cm&fs=1&to=kfrench.eng@gmail.com");
      },
      "l00t" => {
        utils::open_in_new_tab("/dr9lrf26db8ori9");
      },
      "kefahuchi" => {
        utils::open_in_new_tab("/tract-02732f49");
      },
      _ => md::writeln(&term, &format!("{} is not a valid input", target)),
  }
}

pub fn random_george_pic(term: &Terminal) {
  show_random_pic(term, "george", &content::GEORGE_PICS);
}

pub fn random_henry_pic(term: &Terminal) {
  show_random_pic(term, "henry", &content::HENRY_PICS);
}

pub fn random_trout_pic(term: &Terminal) {
  show_random_pic(term, "trout", &content::TROUT_PICS);
}

thread_local! {
  // per command: the pics still to show in the current pass, and the last one
  // shown. every pic is shown once (in a shuffled order) before any repeats.
  static PIC_DECKS: RefCell<HashMap<&'static str, PicDeck>> = RefCell::new(HashMap::new());
}

fn show_random_pic(term: &Terminal, command: &'static str, pics: &[&'static str]) {
  reset_window(&term);

  let filename = PIC_DECKS.with(|decks| {
    decks
      .borrow_mut()
      .entry(command)
      .or_default()
      .next(pics, &mut thread_rng())
      .unwrap_or("")
  });

  let html = format!("<img class='fadeImage' src=/assets/images/{}></img>", filename);

  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_inner_html(&html);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&VISIBLE_CLASS);

  clear(&term);
}

#[derive(Default)]
struct PicDeck {
  remaining: Vec<&'static str>,
  last: Option<&'static str>,
}

impl PicDeck {
  fn next<R: Rng>(&mut self, pics: &[&'static str], rng: &mut R) -> Option<&'static str> {
    if self.remaining.is_empty() {
      self.remaining = pics.iter().copied().filter(|p| !p.trim().is_empty()).collect();
      self.remaining.shuffle(rng);

      // pics are popped off the end, so make sure a new pass doesn't start
      // with the pic that ended the previous one
      let n = self.remaining.len();
      if n > 1 && self.remaining.last().copied() == self.last {
        self.remaining.swap(0, n - 1);
      }
    }

    let pic = self.remaining.pop()?;
    self.last = Some(pic);
    Some(pic)
  }
}

pub fn about(term: &Terminal) {
  reset_window(&term);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&VISIBLE_CLASS);
}

pub fn resume(term: &Terminal, line: &str) {
  let line_split = line.split_ascii_whitespace().collect::<Vec<_>>();
  let sub_cmd = <&str>::clone(&line_split[1]);
  new_line(&term);

  match sub_cmd {
      "help" => subcommand_help_text("resume", "resume edu", &term),
      "languages" | "lang" => {
          md::heading(&term, "languages");
          for s in crate::content::RESUME_LANGUAGES.iter() {
              md::writeln(&term, s);
          }
      },
      "technologies" | "tech" => section(&term, "technologies", &crate::content::RESUME_TECH),
      "experience" | "xp" => experience(&term),
      "education" | "edu" => section(&term, "education", &crate::content::RESUME_EDUCATION),
      "awards" | "awd" => section(&term, "awards", &crate::content::RESUME_AWARDS),
      "publications" => subcommand_help_text("publications", "resume publications books", &term),
      "pub" => subcommand_help_text("publications", "resume pub features", &term),
      _ => md::writeln(&term, &format!("{} is not a valid subcommand for 'resume'", sub_cmd)),
  }
}

fn section(term: &Terminal, title: &str, lines: &[&'static str]) {
  md::heading(&term, title);
  for s in lines.iter() {
    colorize_lede(s, &term);
  }
}

// experience.txt lines look like `NVIDIA|   - NY, NY > staff software engineer   [2026 - present]`
fn parse_experience(line: &str) -> Option<(String, String, String)> {
  let (company, rest) = line.split_once('|')?;
  let rest = rest.trim().trim_start_matches("- ");
  let (role, years) = match rest.rfind('[') {
    Some(i) => (rest[..i].trim(), rest[i..].trim_matches(|c| c == '[' || c == ']' || c == ' ')),
    None => (rest.trim(), ""),
  };
  // drop the location ("NY, NY > ") since it's the same everywhere
  let role = role.split_once(" > ").map(|(_, r)| r).unwrap_or(role);
  Some((company.trim().to_lowercase(), role.to_string(), years.to_string()))
}

fn experience(term: &Terminal) {
  md::heading(&term, "experience");
  let jobs: Vec<(String, String, String)> = crate::content::RESUME_EXPERIENCE.iter().filter_map(|l| parse_experience(l)).collect();

  if utils::is_mobile() {
    // too narrow for a table
    for (company, role, years) in &jobs {
      md::bullet(&term, &Style::new().bold().paint(company.as_str()).to_string());
      md::writeln(&term, &format!("    {}", role));
      md::writeln(&term, &format!("    {}", md::DIM.paint(years.as_str())));
      new_line(&term);
    }
    return;
  }

  let rows: Vec<Vec<String>> = jobs.into_iter().map(|(c, r, y)| vec![c, r, y]).collect();
  md::table(&term, &["company", "role", "years"], &rows, |col, cell| match col {
    0 => Style::new().bold().paint(cell).to_string(),
    _ => cell.to_string(),
  });
}

pub fn colorize_lede(input: &'static str, term: &Terminal) {
  let mut split_line = input.split("|");
  let title = split_line.nth(0).unwrap();
  let mut rest: String = "".to_owned();

  for v in split_line {
    rest.push_str(&v);
  }

  md::writeln(&term, &format!("{} {}", Style::new().bold().paint(title.to_string()).to_string(), &rest));
}

pub fn publications(term: &Terminal, line: &str) {
  let line_split = line.split_ascii_whitespace().collect::<Vec<_>>();
  let sub_cmd = <&str>::clone(&line_split[2]);
  new_line(&term);

  match sub_cmd {
    "books" => {
      let mut iter = crate::content::PUB_BOOKS.iter();
      while let Some(s) = iter.next() {
          colorize_lede(s, &term);
      }
    },
    "features" => {
      let mut iter = crate::content::PUB_FEATURES.iter();
      while let Some(s) = iter.next() {
          colorize_lede(s, &term);
      }
    },
    _ => md::writeln(&term, &format!("{} is not a valid subcommand for 'publications'", sub_cmd)),
  }
}

///// subcommands
pub fn subcommand_help_text(cmd: &str, example: &str, term: &Terminal) {
  reset_window(&term);
  new_line(&term);

  match cmd {
      "resume" => {
        md::heading(&term, "resume");
        resume_help_text(&term);
      },
      "publications" | "pub" => {
        md::heading(&term, "publications");
        publication_help_text(&term);
      },
      _ => md::writeln(&term, &format!("{} is not a valid subcommand", cmd)),
  }

  new_line(&term);
  md::note(&term, &format!("usage: {} <subcommand>, e.g. `{}`", cmd, example));
}

pub fn resume_help_text(term: &Terminal) {
  md::item(&term, "languages", 16, "lang");
  md::item(&term, "technologies", 16, "tech");
  md::item(&term, "experience", 16, "xp");
  md::item(&term, "education", 16, "edu");
  md::item(&term, "awards", 16, "awd");
  md::item(&term, "publications", 16, "pub");
}

pub fn publication_help_text(term: &Terminal) {
  md::item(&term, "books", 12, "published books");
  md::item(&term, "features", 12, "published interviews, essays, & reviews");
}

//// random

/// trying to escalate privileges gets you mr. robot
pub fn is_privesc(input: &str) -> bool {
  const CMDS: &[&str] = &["sudo", "su", "doas", "pkexec", "chmod", "chown", "chgrp", "passwd", "visudo"];
  let words: Vec<&str> = input.split_whitespace().collect();
  let first = words.first().copied().unwrap_or("");
  // `su kathleen` is part of the treasure hunt, not an attack
  let su_to_a_user = first == "su" && words.iter().any(|w| *w == "kathleen" || *w == "stranger");
  (CMDS.contains(&first) && !su_to_a_user) || input.contains("root") || input.contains("/etc/shadow") || input.contains("/etc/sudoers")
}

pub fn throw_hackerman(term: &Terminal) {
  show_image(term, "hackerman.png");
}

/// clear the terminal and show an image from /assets/images above it
pub fn show_image(term: &Terminal, file: &str) {
  reset_window(&term);
  let filepath = format!("/assets/images/{}", file);
  // the page may be scrolled down from earlier output, which would leave only
  // the bottom of the image showing above an empty terminal. scroll to the top
  // now, and again once the image has loaded and taken up its space
  let html = format!("<img src={} onload=\"scrollTo(0, 0)\"></img>", filepath);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_inner_html(&html);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&VISIBLE_CLASS);
  clear(&term);
  web_sys::window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
}

pub fn reset_window(term: &Terminal) {
  web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&HIDDEN);
  clear(&term);
}

pub fn clear(term: &Terminal) {
  // home, erase screen, and erase scrollback (3J). xterm 5 moves erased screen
  // content into scrollback, so without 3J the viewport can stay scrolled down
  // to old output after a clear (easy to hit on phones, where output wraps)
  term.write("\x1b[H\x1b[2J\x1b[3J");
}

pub fn new_line(term: &Terminal) {
  md::writeln(&term, "");
}



pub fn env_text(term: &Terminal) {
  for (k, v) in crate::shell::env_vars() {
    md::writeln(&term, &format!("{}={}", k, v));
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  const PICS: [&str; 4] = ["a.jpg", "b.jpg", "c.jpg", "d.jpg"];

  #[test]
  fn shows_every_pic_before_repeating() {
    let mut deck = PicDeck::default();
    let mut rng = thread_rng();

    for _ in 0..250 {
      let pass: HashSet<_> = (0..PICS.len()).map(|_| deck.next(&PICS, &mut rng).unwrap()).collect();
      assert_eq!(pass.len(), PICS.len());
    }
  }

  #[test]
  fn never_repeats_back_to_back_across_passes() {
    let mut deck = PicDeck::default();
    let mut rng = thread_rng();
    let mut previous = None;

    for _ in 0..1000 {
      let pic = deck.next(&PICS, &mut rng);
      assert_ne!(pic, previous);
      previous = pic;
    }
  }

  #[test]
  fn parses_experience_lines() {
    assert_eq!(
      parse_experience("NVIDIA|                        - NY, NY > staff software engineer                      [2026 - present]"),
      Some(("nvidia".into(), "staff software engineer".into(), "2026 - present".into())),
    );
    assert_eq!(
      parse_experience("GROQ|  - NY, NY > staff software engineer > inference  [2025 - 2026]"),
      Some(("groq".into(), "staff software engineer > inference".into(), "2025 - 2026".into())),
    );
    assert_eq!(parse_experience(""), None);
  }

  #[test]
  fn handles_single_and_empty_lists() {
    let mut rng = thread_rng();
    let mut deck = PicDeck::default();
    assert_eq!(deck.next(&["only.jpg"], &mut rng), Some("only.jpg"));
    assert_eq!(deck.next(&["only.jpg"], &mut rng), Some("only.jpg"));
    assert_eq!(PicDeck::default().next(&["", "  "], &mut rng), None);
  }
}
