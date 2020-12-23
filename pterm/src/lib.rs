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
    // "botnet",
    // "about",
];

use futures::lock::Mutex;
use instant::Instant;
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32};
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions, Theme};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window};
use colored::*;

use app::AppConfig;
use rand::thread_rng;

use ansi_term::{Colour, Style};

use crate::io::{csleep, delayed_print, new_line, print, clear_line};

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

fn prompt(term: &Terminal) {
    term.writeln("");
    term.write(&PROMPT);
}

fn help_text(term: &Terminal) {
    term.writeln("");
    term.writeln("[COMMANDS]:");
    term.writeln("");
    term.writeln("about:          learn more about me");
    term.writeln("resume:         view my resume");
    term.writeln("projects:       see various projects i've worked on");
    term.writeln("contact:        contact me");
    term.writeln("clear:          clear the terminal window");
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

// https://notes.burke.libbey.me/ansi-escape-codes/
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_LEFT_ARROW: u32 = 37;
const KEY_RIGHT_ARROW: u32 = 39;
const KEY_C: u32 = 67;
const KEY_L: u32 = 76;

const CURSOR_LEFT: &str = "\x1b[D";
const CURSOR_RIGHT: &str = "\x1b[C";

const SHRINK_CLASS: &str = "shrink";
const VISIBLE_CLASS: &str = "visible";
const HIDDEN: &str = "hidden";

pub async fn run_intro(cfg: &AppConfig) {
    sections::intro::run(cfg).await;
}

pub async fn run(cfg: AppConfig, el: Element) {
    let mut thread_range = thread_rng();

    console_log!("RUN EL: {:?}", el);
    el.set_class_name(&SHRINK_CLASS);

    sections::downloads::run(&cfg).await;

    // loop {
    //     let choice: &str = cfg.sections.choose(&mut thread_range).unwrap();
    //     match choice {
    //         "faux_downloads" => sections::downloads::run(&cfg).await,
    //         // "botnet" => sections::botnet::run(&cfg).await,
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

// fn setup_intro_term(window: &Window, document: &Document) -> Result<(), JsValue> {
//     let intro_term = document.get_element_by_id("intro-term").expect("should have #intro-term on the page");

//     let a = Closure::wrap(Box::new(move || ))

//     fn update_term(current_term: &Element) {
//         current_term.set_inner_html(&String::from(

//         ));
//     }
// }

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // let window = web_sys::window().expect("should have a window in this context");
    // let document = window.document().expect("window should have a document");

    let cfg = app::parse_inputs();
    *SPEED_FACTOR.lock().await = cfg.speed_factor;

    run_intro(&cfg).await;

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
                    console_log!("LINE: {}", line);

                    let mut line_match: &str = &line;

                    match line_match {
                        "help" => {
                            help_text(&term);
                        },
                        "about" => {
                            // show the about section
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&VISIBLE_CLASS);

                            term.writeln("more about me");
                        },
                        "resume" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);

                            term.writeln("my resume");
                        },
                        "projects" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);

                            term.writeln("my projects...");
                        },
                        "contact" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);

                            term.writeln("contact me @...");
                        },
                        "clear" => {
                            // hide any visible sections
                            web_sys::window().unwrap().document().unwrap().get_element_by_id("about").unwrap().set_class_name(&HIDDEN);

                            // clear all term input text
                            term.write("\x1b[H\x1b[2J");
                        },
                        _ => {
                            term.writeln(&format!("'{}' is not a valid command! run 'help' to list all valid commands", line));
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