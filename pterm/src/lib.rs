pub mod app;

mod io;
mod content;
mod sections;
mod utils;
mod term;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub static PORTFOLIO_SECTIONS: &[&str] = &[
    "faux_downloads",
    "resume",
    "botnet",
];

use futures::lock::Mutex;
use instant::Instant;
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32};
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions, Theme};
use xterm_js_rs::addons::fit::FitAddon;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlElement, Window, Location};
use colored::*;

use app::AppConfig;
use rand::thread_rng;

use ansi_term::{Colour, Style};

use crate::io::{csleep, delayed_print, new_line, print, clear_line};
use crate::content::{RESUME_AWARDS, RESUME_EDUCATION, RESUME_EXPERIENCE, RESUME_LANGUAGES, RESUME_TECH};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

lazy_static::lazy_static! {
    pub static ref START_TIME: Instant = Instant::now();
}

lazy_static::lazy_static! {
    pub static ref SPEED_FACTOR: Mutex<f32> = Mutex::new(1.0);
}

lazy_static::lazy_static! {
    pub static ref SECTIONS_SHOWN: AtomicU32 = AtomicU32::new(0);
}

lazy_static::lazy_static! {
    pub static ref CTRLC_PRESSED: AtomicBool = AtomicBool::new(false);
}

pub async fn run_intro(cfg: &AppConfig) {
    sections::intro::run(cfg).await;
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn run(cfg: AppConfig) {
    let mut thread_range = thread_rng();

    loop {
        let choice: &str = cfg.sections.choose(&mut thread_range).unwrap();
        match choice {
            "faux_downloads" => sections::downloads::run(&cfg).await,
            "botnet" => sections::botnet::run(&cfg).await,
            _ => print!("fix me later"),
            // _ => panic!("unknown section '{}'!", choice),
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::sync::atomic::Ordering;
            SECTIONS_SHOWN.fetch_add(1, Ordering::SeqCst);

            if cfg.should_quit() {
                quit();
            }
        }
    }
}


#[cfg(target_arch = "wasm32")]
pub async fn run(cfg: AppConfig, intro_animation: Element) {
    intro_animation.set_class_name(&crate::term::HIDDEN);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn quit() {
    std::process::exit(0);
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let cfg = app::parse_inputs();

    *SPEED_FACTOR.lock().await = cfg.speed_factor;

    // run_intro(&cfg).await;

    let terminal: Terminal = Terminal::new(
        TerminalOptions::new()
        .with_rows(50)
        .with_cols(150)
        .with_cursor_blink(true)
        .with_cursor_width(10)
        .with_font_size(12)
        .with_draw_bold_text_in_bright_colors(true)
        .with_right_click_selects_word(true),
    );

    let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id("terminal").unwrap();
    let intro_elem = web_sys::window().unwrap().document().unwrap().get_element_by_id("intro-terminal").unwrap();

    crate::term::help_text(&terminal);
    terminal.open(elem.dyn_into()?);
    crate::term::prompt(&terminal);

    let mut line = String::new();
    let mut cursor_col = 0;
    let term: Terminal = terminal.clone().dyn_into()?;

    let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
        let event = e.dom_event();

        match event.key_code() {
            crate::term::KEY_ENTER => {
                if !line.is_empty() {
                    crate::term::new_line(&term);
                    let mut line_match: &str = &line.trim();
                    match line_match {
                        "help" => crate::term::help_text(&term),
                        "about" => crate::term::about(&term),
                        "resume" => crate::term::subcommand_help_text("resume", "resume xp", &term),
                        "projects" => crate::term::projects(&term),
                        "contact" => crate::term::contact_info(&term),
                        "clear" => crate::term::reset_window(&term),
                        "whoami" => crate::term::whoami(&term),
                        "george" => crate::term::random_george_pic(&term),
                        "git" => crate::term::throw_git_error(&term),
                        "sudo" => crate::term::throw_hackerman(&term),
                        "pwd" => term.writeln("/home/stranger"),
                        "ls" => {
                            term.writeln("Documents");
                            term.writeln("Downloads");
                            term.writeln("Pictures");
                        },
                        "cat" => crate::term::permission_denied(&term),
                        _ => {
                            if crate::term::deny_common_bins(line_match) {
                                crate::term::permission_denied(&term);
                            } else if crate::term::should_throw_hackerman(line_match) {
                                crate::term::throw_hackerman(&term);
                            } else if line_match.contains("ls ") && line_match.contains("/") {
                                crate::term::permission_denied(&term);
                            } else if line_match.contains("ls -") {
                                term.writeln("Documents");
                                term.writeln("Downloads");
                                term.writeln("Pictures");
                                term.writeln(".ssh");
                                term.writeln(".bashrc");
                                term.writeln(".vimrc");
                            } else if line_match.contains("git ") {
                                term.writeln(&format!("{}", Colour::Red.bold().paint("fatal: This operation must be run in a work tree").to_string()));
                            } else if line_match.contains("resume ") {
                                let line_split = line_match.split_ascii_whitespace().collect::<Vec<_>>();
                                let sub_cmd = <&str>::clone(&line_split[1]);
                                crate::term::new_line(&term);

                                match sub_cmd {
                                    "help" => crate::term::subcommand_help_text("resume", "resume edu", &term),
                                    "pdf" => {
                                        utils::open_in_new_tab("/assets/files/resume.pdf");
                                    },
                                    "languages" => {
                                        let mut iter = RESUME_LANGUAGES.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "lang" => {
                                        let mut iter = RESUME_LANGUAGES.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "technologies" => {
                                        let mut iter = RESUME_TECH.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "tech" => {
                                        let mut iter = RESUME_TECH.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "experience" => {
                                        let mut iter = RESUME_EXPERIENCE.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "xp" => {
                                        let mut iter = RESUME_EXPERIENCE.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "education" => {
                                        let mut iter = RESUME_EDUCATION.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "edu" => {
                                        let mut iter = RESUME_EDUCATION.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "awards" => {
                                        let mut iter = RESUME_AWARDS.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "awd"  => {
                                        let mut iter = RESUME_AWARDS.iter();
                                        while let Some(s) = iter.next() {
                                            term.writeln(s);
                                        }
                                    },
                                    "publications" => term.writeln("TK"),
                                    "pub" => term.writeln("TK"),
                                    _ => term.writeln(&format!("{} is not a valid subcommand for 'resume'", sub_cmd)),
                                }
                            } else if line_match.contains("goto ") {
                                let line_split = line_match.split_ascii_whitespace().collect::<Vec<_>>();
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
                                    _ => term.writeln(&format!("{} is not a valid input", target)),
                                }
                            } else {
                                term.writeln(&format!("command not found: '{}'", line));
                            }
                        },
                    }

                    line.clear();
                    cursor_col = 0;
                }

                crate::term::prompt(&term);
            }
            crate::term::KEY_BACKSPACE => {
                if cursor_col > 0 {
                    term.write("\u{0008} \u{0008}");
                    line.pop();
                    cursor_col -= 1;
                }
            }
            crate::term::KEY_LEFT_ARROW => {
                if cursor_col > 0 {
                    term.write(crate::term::CURSOR_LEFT);
                    cursor_col -= 1;
                }
            }
            crate::term::KEY_RIGHT_ARROW => {
                if cursor_col < line.len() {
                    term.write(crate::term::CURSOR_RIGHT);
                    cursor_col += 1;
                }
            }
            crate::term::KEY_L if event.ctrl_key() => term.writeln(""),
            crate::term::KEY_C if event.ctrl_key() => {
                crate::term::prompt(&term);
                line.clear();
                cursor_col = 0;
            }
            crate::term::KEY_U if event.ctrl_key() => {
                crate::term::prompt(&term);
                line.clear();
                cursor_col = 0;
            }
            _ => {
                if !event.alt_key() && !event.alt_key() && !event.ctrl_key() && !event.meta_key() {
                    term.write(&event.key());
                    line.push_str(&e.key());
                    cursor_col += 1;
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    terminal.on_key(callback.as_ref().unchecked_ref());

    callback.forget();

    let addon = FitAddon::new();
    terminal.load_addon(addon.clone().dyn_into::<FitAddon>()?.into());
    addon.fit();
    terminal.focus();

    run(cfg, intro_elem).await;

    Ok(())
}