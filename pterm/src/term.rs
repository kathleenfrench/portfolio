use xterm_js_rs::Terminal;
use ansi_term::{Colour, Style};
use rand::thread_rng;
use rand::prelude::*;

use crate::content;
use crate::utils;

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
  term.writeln("replay         replay the intro animation");
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

pub fn goto_links(term: &Terminal, line: &str) {
  let line_split = line.split_ascii_whitespace().collect::<Vec<_>>();
  let target = <&str>::clone(&line_split[1]);

  match target {
      "github" => {
          term.writeln(&format!("redirecting to {}...", target));
          utils::open_in_new_tab("https://github.com/kathleenfrench");
      },
      "linkedin" => {
          term.writeln(&format!("redirecting to {}...", target));
          utils::open_in_new_tab("https://www.linkedin.com/in/frenchkathleen/");
      },
      "email" => {
          utils::open_in_new_tab("https://mail.google.com/mail/?view=cm&fs=1&to=kfrench09@gmail.com");
      },
      "l00t" => {
        utils::open_in_new_tab("/dr9lrf26db8ori9");
      },
      _ => term.writeln(&format!("{} is not a valid input", target)),
  }
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

pub fn resume(term: &Terminal, line: &str) {
  let line_split = line.split_ascii_whitespace().collect::<Vec<_>>();
  let sub_cmd = <&str>::clone(&line_split[1]);
  new_line(&term);

  match sub_cmd {
      "help" => subcommand_help_text("resume", "resume edu", &term),
      "pdf" => {
          utils::open_in_new_tab("/assets/files/resume/resume.pdf");
      },
      "languages" => {
          let mut iter = crate::content::RESUME_LANGUAGES.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "lang" => {
          let mut iter = crate::content::RESUME_LANGUAGES.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "technologies" => {
          let mut iter = crate::content::RESUME_TECH.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "tech" => {
          let mut iter = crate::content::RESUME_TECH.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "experience" => {
          let mut iter = crate::content::RESUME_EXPERIENCE.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "xp" => {
          let mut iter = crate::content::RESUME_EXPERIENCE.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "education" => {
          let mut iter = crate::content::RESUME_EDUCATION.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "edu" => {
          let mut iter = crate::content::RESUME_EDUCATION.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "awards" => {
          let mut iter = crate::content::RESUME_AWARDS.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "awd"  => {
          let mut iter = crate::content::RESUME_AWARDS.iter();
          while let Some(s) = iter.next() {
              term.writeln(s);
          }
      },
      "publications" => term.writeln("TK"),
      "pub" => term.writeln("TK"),
      _ => term.writeln(&format!("{} is not a valid subcommand for 'resume'", sub_cmd)),
  }
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
  if input.contains("cat ") && input != "cat .topsecret" || input.contains("cd ") || input.contains("ps ") || input.contains("/bin") {
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

pub fn should_deny_ls(input: &str) -> bool {
  if input.contains("ls ") {
    if input.contains("/") && !input.contains("/tmp") || input.contains("Documents") || input.contains("Downloads") || input.contains("Pictures") {
      return true
    }
  }

  false
}

pub fn should_ls_top_secret(input: &str) -> bool {
  if input.contains("ls ") && input.contains("/tmp") {
    return true
  }

  false
}

pub fn ls(term: &Terminal, show_dotfiles: bool) {
  if show_dotfiles {
    term.writeln("Documents");
    term.writeln("Downloads");
    term.writeln("Pictures");
    term.writeln(".ssh");
    term.writeln(".bashrc");
    term.writeln(".vimrc");
  } else {
    term.writeln("Documents");
    term.writeln("Downloads");
    term.writeln("Pictures");
  }
}

pub fn top_secret(term: &Terminal) {
  term.writeln("nice work :)");
  term.writeln("run 'goto l00t' for a surprise...");
  new_line(&term);
}

pub fn command_not_found(term: &Terminal, line: &str) {
  term.writeln(&format!("command not found: '{}'", line));
}

pub fn echo(term: &Terminal, line: &str) {
  let mut stripped = line.replace("echo", "");
  term.writeln(&stripped.replace("'", "").replace('"', "").trim());
}

pub fn env_text(term: &Terminal) {
  term.writeln("EDITOR=vim");
  term.writeln("HOME=/Users/stranger");
  term.writeln("LANG=en_US.UTF-8");
  term.writeln("LOGNAME=stranger");
  term.writeln("PATH=/usr/local/bin");
  term.writeln("PWD=/Users/stranger");
  term.writeln("USER=stranger");
  term.writeln("SHELL=/bin/sh");
  term.writeln("TMPDIR=/var/folders/1_/hzsf_8wsg8sjdfjghasg/T/");
}