use xterm_js_rs::Terminal;
use ansi_term::{Colour, Style};
use crate::content;
use rand::thread_rng;
use rand::prelude::*;

pub const KEY_ENTER: u32 = 13;
pub const KEY_BACKSPACE: u32 = 8;
pub const KEY_LEFT_ARROW: u32 = 37;
pub const KEY_RIGHT_ARROW: u32 = 39;
pub const KEY_C: u32 = 67;
pub const KEY_L: u32 = 76;
pub const KEY_U: u32 = 85;

pub const CURSOR_LEFT: &str = "\x1b[D";
pub const CURSOR_RIGHT: &str = "\x1b[C";

pub const VISIBLE_CLASS: &str = "visible";
pub const HIDDEN: &str = "hidden";

lazy_static::lazy_static! {
  pub static ref PROMPT: String = Colour::Yellow.bold().paint("kathleenfrench@portfolio $ ").to_string();
}

lazy_static::lazy_static! {
  pub static ref PERMISSION_DENIED_ERR: String = Colour::Red.bold().paint("Permission denied").to_string();
}

pub fn prompt(term: &Terminal) {
  term.writeln("");
  term.write(&PROMPT);
}

pub fn permission_denied(term: &Terminal) {
  term.write(&PERMISSION_DENIED_ERR);
  term.writeln("");
}

///// commands
pub fn help_text(term: &Terminal) {
  reset_window(&term);
  new_line(&term);
  term.writeln("[COMMANDS]:");
  new_line(&term);
  term.writeln(&format!("{}", Colour::Green.bold().paint("type a command and hit the 'Enter' key to execute").to_string()));
  new_line(&term);
  term.writeln("about          learn more about me");
  term.writeln("resume         view available subcommands");
  term.writeln("projects       see various projects i've worked on");
  term.writeln("george         show a random picture of my dog");
  term.writeln("contact        contact me");
  term.writeln("clear          clear the terminal window");
  new_line(&term);
  term.writeln(&format!("{}", Colour::Green.bold().paint("run `help` at any point to show this screen").to_string()));
  new_line(&term);
}

pub fn contact_info(term: &Terminal) {
  reset_window(&term);
  new_line(&term);
  term.writeln(&format!("run '{}' from the list below to launch a separate service", Colour::Green.bold().paint("goto <link>").to_string()));
  term.writeln(&format!("[example]: {}", Colour::Blue.bold().paint("goto github").to_string()));
  new_line(&term);
  term.writeln("- github");
  term.writeln("- linkedin");
  term.writeln("- email");
  new_line(&term);
}

pub fn random_george_pic(term: &Terminal) {
  reset_window(&term);

  let mut rng = thread_rng();
  let filename = &content::GEORGE_PICS.choose(&mut rng).unwrap_or(&"");
  let filepath = format!("/assets/images/{}", filename);
  let html = format!("<img src={}></img>", filepath);

  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_inner_html(&html);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&VISIBLE_CLASS);

  clear(&term);
}

pub fn about(term: &Terminal) {
  reset_window(&term);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&VISIBLE_CLASS);
  term.writeln("~ moi ~");
}

pub fn projects(term: &Terminal) {
  reset_window(&term);
  term.writeln("my projects...");
}

///// subcommands
pub fn subcommand_help_text(cmd: &str, example: &str, term: &Terminal) {
  reset_window(&term);

  new_line(&term);
  term.writeln("[SUBCOMMANDS]:");
  new_line(&term);
  term.writeln(&format!("{}: {} <subcommand>", Colour::Green.bold().paint("[usage]").to_string(), cmd));
  term.writeln(&format!("{}: {}", Colour::Green.bold().paint("[example]").to_string(), example));
  new_line(&term);

  match cmd {
      "resume" => {
          term.writeln(&format!("pdf           - download the full resume in pdf form"));
          term.writeln(&format!("languages     ({})", Colour::Blue.bold().paint("lang").to_string()));
          term.writeln(&format!("technologies  ({})", Colour::Blue.bold().paint("tech").to_string()));
          term.writeln(&format!("experience    ({})", Colour::Blue.bold().paint("xp").to_string()));
          term.writeln(&format!("education     ({})", Colour::Blue.bold().paint("edu").to_string()));
          term.writeln(&format!("awards        ({})", Colour::Blue.bold().paint("awd").to_string()));
          term.writeln(&format!("publications  ({})", Colour::Blue.bold().paint("pub").to_string()));
      }
      _ => term.writeln(&format!("{} is not a valid subcommand", cmd)),
  }

  new_line(&term);
  new_line(&term);
}

//// random

pub fn should_throw_hackerman(input: &str) -> bool {
  if input.contains("root") || input.contains("sudo ") || input.contains("chown ") || input.contains("chmod ") || input.contains("which ") {
    return true;
  }

  false
}

pub fn throw_hackerman(term: &Terminal) {
  reset_window(&term);
  let filepath = "/assets/images/hackerman.png";
  let html = format!("<img src={}></img>", filepath);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_inner_html(&html);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&VISIBLE_CLASS);
  clear(&term);
}

pub fn deny_common_bins(input: &str) -> bool {
  if input.contains("cat ") || input.contains("cd ") || input.contains("ps ") {
    return true;
  }

  false
}

pub fn throw_git_error(term: &Terminal) {
  term.writeln(&format!("{}", Colour::Red.bold().paint("fatal: This operation must be run in a work tree").to_string()));
}

pub fn reset_window(term: &Terminal) {
  web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);
  web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&HIDDEN);
  clear(&term);
}

pub fn clear(term: &Terminal) {
  term.write("\x1b[H\x1b[2J");
}

pub fn new_line(term: &Terminal) {
  term.writeln("");
}

pub fn whoami(term: &Terminal) {
  term.writeln("stranger (danger)");
}