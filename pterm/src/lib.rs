pub mod app;

mod io;
mod content;
mod sections;
mod utils;

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
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlElement, Window, Location};
use colored::*;

use app::AppConfig;
use rand::thread_rng;

use ansi_term::{Colour, Style};

use crate::io::{csleep, delayed_print, new_line, print, clear_line};
use crate::content::{GEORGE_PICS, RESUME_AWARDS, RESUME_EDUCATION, RESUME_EXPERIENCE, RESUME_LANGUAGES, RESUME_TECH};

#[wasm_bindgen]
pub extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

lazy_static::lazy_static! {
    pub static ref PROMPT: String = Colour::Yellow.bold().paint("kathleenfrench@portfolio $ ").to_string();
}

lazy_static::lazy_static! {
    pub static ref PERMISSION_DENIED_ERR: String = Colour::Red.bold().paint("Permission denied").to_string();
}

fn prompt(term: &Terminal) {
    term.writeln("");
    term.write(&PROMPT);
}

fn permission_denied(term: &Terminal) {
    term.write(&PERMISSION_DENIED_ERR);
    term.writeln("");
}

fn help_text(term: &Terminal) {
    term.writeln("");
    term.writeln("[COMMANDS]:");
    term.writeln("");
    term.writeln(&format!("{}", Colour::Green.bold().paint("type a command and hit the 'Enter' key to execute").to_string()));
    term.writeln("");
    term.writeln("about:          learn more about me");
    term.writeln("resume:         view available subcommands");
    term.writeln("projects:       see various projects i've worked on");
    term.writeln("george:         show a random picture of my dog");
    term.writeln("contact:        contact me");
    term.writeln("clear:          clear the terminal window");
    term.writeln("");
    term.writeln("");
}

fn contact_info(term: &Terminal) {
    term.writeln("");
    term.writeln(&format!("run '{}' from the list below to launch a separate service", Colour::Green.bold().paint("goto <link>").to_string()));
    term.writeln(&format!("[example]: {}", Colour::Blue.bold().paint("goto github").to_string()));
    term.writeln("");
    term.writeln("- github");
    term.writeln("- linkedin");
    term.writeln("- email");
    term.writeln("");
}

fn subcommand_help_text(cmd: &str, example: &str, term: &Terminal) {
    term.writeln("");
    term.writeln("[SUBCOMMANDS]:");
    term.writeln("");
    term.writeln(&format!("{}: {} <subcommand>", Colour::Green.bold().paint("[usage]").to_string(), cmd));
    term.writeln(&format!("{}: {}", Colour::Green.bold().paint("[example]").to_string(), example));
    term.writeln("");

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

    term.writeln("");
    term.writeln("");
}

fn testing(term: &Terminal) {
    term.write("\x1b[H\x1b[2J");
    term.writeln("HELLO THIS IS A TEST");
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

// https://theasciicode.com.ar/
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_LEFT_ARROW: u32 = 37;
const KEY_RIGHT_ARROW: u32 = 39;
const KEY_C: u32 = 67;
const KEY_L: u32 = 76;
const KEY_U: u32 = 85;

const CURSOR_LEFT: &str = "\x1b[D";
const CURSOR_RIGHT: &str = "\x1b[C";

const SHRINK_CLASS: &str = "shrink";
const VISIBLE_CLASS: &str = "visible";
const HIDDEN: &str = "hidden";

pub async fn run_intro(cfg: &AppConfig) {
    sections::intro::run(cfg).await;
}

pub async fn run(cfg: AppConfig, intro_animation: Element) {
    // let mut thread_range = thread_rng();

    intro_animation.set_class_name(&HIDDEN);

    // loop {
    //     let choice: &str = cfg.sections.choose(&mut thread_range).unwrap();
    //     match choice {
    //         "faux_downloads" => sections::downloads::run(&cfg).await,
    //         "botnet" => sections::botnet::run(&cfg).await,
    //         _ => print!("fix me later"),
    //         // _ => panic!("unknown section '{}'!", choice),
    //     }

    //     #[cfg(not(target_arch = "wasm32"))]
    //     {
    //         use std::sync::atomic::Ordering;
    //         SECTIONS_SHOWN.fetch_add(1, Ordering::SeqCst);

    //         if cfg.should_quit() {
    //             quit();
    //         }
    //     }
    // }
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
        .with_cols(100)
        .with_cursor_blink(true)
        .with_cursor_width(10)
        .with_font_size(12)
        .with_draw_bold_text_in_bright_colors(true)
        .with_right_click_selects_word(true),
    );

    let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id("terminal").unwrap();
    let intro_elem = web_sys::window().unwrap().document().unwrap().get_element_by_id("intro-terminal").unwrap();

    help_text(&terminal);
    terminal.open(elem.dyn_into()?);
    prompt(&terminal);

    let mut line = String::new();
    let mut cursor_col = 0;

    let term: Terminal = terminal.clone().dyn_into()?;

    let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
        let event = e.dom_event();

        match event.key_code() {
            KEY_ENTER => {
                if !line.is_empty() {
                    term.writeln("");

                    let mut line_match: &str = &line;

                    match line_match {
                        "help" => {
                            help_text(&term);
                        },
                        "about" => {
                            // show the about section
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&VISIBLE_CLASS);
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&HIDDEN);

                            term.write("\x1b[H\x1b[2J");
                            term.writeln("~ moi ~");
                        },
                        "resume" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&HIDDEN);

                            term.write("\x1b[H\x1b[2J");
                            subcommand_help_text("resume", "resume xp", &term);
                        },
                        "projects" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&HIDDEN);

                            term.writeln("my projects...");
                        },
                        "contact" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&HIDDEN);

                            contact_info(&term);
                        },
                        "clear" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&HIDDEN);

                            // clear all term input text
                            term.write("\x1b[H\x1b[2J");
                        },
                        "whoami" => {
                            term.writeln("idk");
                            term.writeln("...or do i?");
                        },
                        "george" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);

                            let mut rng = thread_rng();
                            let mut filename = &GEORGE_PICS.choose(&mut rng).unwrap_or(&"");
                            let filepath = format!("/assets/images/{}", filename);
                            let html = format!("<img src={}></img>", filepath);

                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_inner_html(&html);
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&VISIBLE_CLASS);

                            term.write("\x1b[H\x1b[2J");
                        },
                        "sudo" => {
                            let filepath = "/assets/images/hackerman.png";
                            let html = format!("<img src={}></img>", filepath);
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_inner_html(&html);
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&VISIBLE_CLASS);

                            term.write("\x1b[H\x1b[2J");
                        },
                        "pwd" => {
                            term.writeln("/home/stranger");
                        },
                        "ls" => {
                            term.writeln("Documents");
                            term.writeln("Downloads");
                            term.writeln("Pictures");
                        },
                        "cat" => {
                            permission_denied(&term);
                        }
                        _ => {
                            if line_match.contains("cat ") || line_match.contains("cd ") {
                                permission_denied(&term);
                            } else if line_match.contains("ls -") {
                                term.writeln("Documents");
                                term.writeln("Downloads");
                                term.writeln("Pictures");
                                term.writeln(".ssh");
                                term.writeln(".bashrc");
                                term.writeln(".vimrc");
                            } else if line_match.contains("sudo ") || line_match.contains("chown ") || line_match.contains("chmod ") {
                                let filepath = "/assets/images/hackerman.png";
                                let html = format!("<img src={}></img>", filepath);
                                web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_inner_html(&html);
                                web_sys::window().unwrap().document().unwrap().get_element_by_id("content").unwrap().set_class_name(&VISIBLE_CLASS);
                                term.write("\x1b[H\x1b[2J");
                            } else if line_match.contains("resume ") {
                                let line_split = line_match.split_ascii_whitespace().collect::<Vec<_>>();
                                let sub_cmd = <&str>::clone(&line_split[1]);
                                term.writeln("");

                                match sub_cmd {
                                    "help" => subcommand_help_text("resume", "resume edu", &term),
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
                                    "publications" => term.writeln("test"),
                                    "pub" => term.writeln("test"),
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
                                term.writeln(&format!("'{}' is not a valid command! run 'help' to list all valid commands", line));
                            }
                        },
                    }

                    line.clear();
                    cursor_col = 0;
                }
                prompt(&term);
            }
            KEY_BACKSPACE => {
                if cursor_col > 0 {
                    term.write("\u{0008} \u{0008}");
                    line.pop();
                    cursor_col -= 1;
                }
            }
            KEY_LEFT_ARROW => {
                if cursor_col > 0 {
                    term.write(CURSOR_LEFT);
                    cursor_col -= 1;
                }
            }
            KEY_RIGHT_ARROW => {
                if cursor_col < line.len() {
                    term.write(CURSOR_RIGHT);
                    cursor_col += 1;
                }
            }
            KEY_L if event.ctrl_key() => term.writeln(""),
            KEY_C if event.ctrl_key() => {
                prompt(&term);
                line.clear();
                cursor_col = 0;
            }
            KEY_U if event.ctrl_key() => {
                prompt(&term);
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
    terminal.focus();

    run(cfg, intro_elem).await;

    Ok(())
}