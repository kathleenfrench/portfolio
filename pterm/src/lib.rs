pub mod app;

mod io;
mod content;
mod sections;
mod utils;

pub static PORTFOLIO_SECTIONS: &[&str] = &[
    "intro",
    "resume",
    "botnet",
    // "about",
];

use futures::lock::Mutex;
use instant::Instant;
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32};
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions, Theme};
use wasm_bindgen::JsCast;

use app::AppConfig;
use rand::thread_rng;

use crate::io::{csleep, delayed_print, new_line, print, clear_line};

const PROMPT: &str = "$ ";

fn prompt(term: &Terminal) {
    term.writeln("");
    term.write(PROMPT);
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

// Keyboard keys
// https://notes.burke.libbey.me/ansi-escape-codes/
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_LEFT_ARROW: u32 = 37;
const KEY_RIGHT_ARROW: u32 = 39;
const KEY_C: u32 = 67;
const KEY_L: u32 = 76;

const CURSOR_LEFT: &str = "\x1b[D";
const CURSOR_RIGHT: &str = "\x1b[C";

pub async fn run_intro(cfg: &AppConfig) {
    sections::intro::run(cfg).await;
}

pub async fn run(cfg: AppConfig) {
    let mut thread_range = thread_rng();

    // for l in cfg.sections.iter() {
    //     match l as &str {
    //         "intro" => sections::intro::run(&cfg).await,
    //         "resume" => sections::resume::run(&cfg).await,
    //         "botnet" => print!("hi"),
    //         _ => panic!("unknown section '{}'!", l),
    //     }
    // }

    loop {
        let choice: &str = cfg.sections.choose(&mut thread_range).unwrap();
        match choice {
            // "intro" => {
            //     sections::intro::run(&cfg).await;
            //     sections::resume::run(&cfg).await;
            //     return
            // },
            // "resume",
            // "about",
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

    run_intro(&cfg).await;

    let terminal: Terminal = Terminal::new(
        TerminalOptions::new()
        .with_rows(50)
        .with_cursor_blink(true)
        .with_cursor_width(10)
        .with_font_size(12)
        .with_draw_bold_text_in_bright_colors(true)
        .with_right_click_selects_word(true)
        .with_theme(
            Theme::new()
                .with_foreground("#29FF00"),
                // .with_background("#000000"),
        ),
    );

    let elem = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("terminal")
        .unwrap();

    terminal.writeln("Supported keys in this example: <Printable-Characters> <Enter> <Backspace> <Left-Arrow> <Right-Arrow> <Ctrl-C> <Ctrl-L>");
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
                    term.writeln(&format!("You entered {} characters '{}'", line.len(), line));
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

    run(cfg).await;

    Ok(())
}