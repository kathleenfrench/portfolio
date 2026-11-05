pub mod app;

mod io;
mod content;
mod md;
mod shell;
mod fun;
mod fx;
mod sections;
mod utils;
mod term;
mod local;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO
pub static PORTFOLIO_SECTIONS: &[&str] = &[
    "faux_downloads",
];

use futures::lock::Mutex;
use instant::Instant;
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32};
#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions};
use xterm_js_rs::addons::fit::FitAddon;
use wasm_bindgen::JsCast;
use web_sys::Element;
use app::AppConfig;

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
            _ => print!("nope"),
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
pub async fn run(_cfg: AppConfig, intro_animation: Element) {
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
    utils::desktop_or_mobile();

    let mut local = crate::local::Store::new();
    let cfg = app::parse_inputs();

    *SPEED_FACTOR.lock().await = cfg.speed_factor;

    if !local.get_intro_played() {
        run_intro(&cfg).await;
    }

    local.set_intro_played(true);
    local.save_to_local_storage();

    let mut font_size: u32 = 12;
    let mut cols: u32 = 150;

    if utils::is_mobile() {
        font_size = 11;
        cols = 60;
    }

    let options = TerminalOptions::new();
    options
        .with_rows(50)
        .with_cols(cols)
        .with_cursor_blink(true)
        .with_cursor_width(10)
        .with_font_size(font_size)
        .with_draw_bold_text_in_bright_colors(true)
        .with_right_click_selects_word(true);

    // keep the cursor a solid block when the terminal isn't focused (xterm 5.3+,
    // not in xterm-js-rs's builder). phones don't focus it until you tap, and the
    // default there is a hollow outline. the blink itself is in main.css
    js_sys::Reflect::set(&options, &"cursorInactiveStyle".into(), &"block".into())?;
    // same font as the rest of the site (main.js waits for it to load first)
    js_sys::Reflect::set(&options, &"fontFamily".into(), &"'IBM Plex Mono', monospace".into())?;

    // softer than pure white on black. keep in sync with main.js + main.css
    let theme = js_sys::Object::new();
    js_sys::Reflect::set(&theme, &"background".into(), &"#111111".into())?;
    js_sys::Reflect::set(&theme, &"foreground".into(), &"#e6e6e6".into())?;
    js_sys::Reflect::set(&theme, &"cursor".into(), &"#e6e6e6".into())?;
    js_sys::Reflect::set(&options, &"theme".into(), &theme)?;

    let terminal: Terminal = Terminal::new(&options);

    let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id("terminal").unwrap();
    let intro_elem = web_sys::window().unwrap().document().unwrap().get_element_by_id("intro-terminal").unwrap();

    crate::shell::load();
    crate::term::welcome(&terminal);
    terminal.open(elem.dyn_into()?);
    crate::term::prompt(&terminal);

    // the line being typed, the cursor position in it (in chars), and which
    // typed command the up arrow is showing (None = the line being typed).
    // shared by the key handler and the paste handler
    let editor: Rc<RefCell<(String, usize, Option<usize>)>> = Rc::new(RefCell::new((String::new(), 0, None)));
    let term: Terminal = terminal.clone().dyn_into()?;
    let addon = FitAddon::new();

    let key_editor = editor.clone();
    let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
        let event = e.dom_event();

        // top / the meltdown own the terminal while they run: q or ctrl+c stops top
        if crate::fx::is_busy() {
            let key = event.key();
            if key == "q" || (event.ctrl_key() && event.key_code() == crate::term::KEY_C) {
                crate::fx::stop();
            }
            return;
        }

        let mut editing = key_editor.borrow_mut();
        let (line, cursor_col, recall) = &mut *editing;

        match event.key_code() {
            crate::term::KEY_ENTER => {
                // jump to the end of the line first: clearing the hint erases
                // everything after the cursor
                let len = line.chars().count();
                if *cursor_col < len && !crate::shell::is_awaiting_password() {
                    term.write(&format!("\x1b[{}C", len - *cursor_col));
                }
                crate::term::clear_hint(&term);

                // the line after `su kathleen`'s Password: prompt
                if crate::shell::is_awaiting_password() {
                    crate::term::new_line(&term);
                    for l in crate::shell::submit_password(&line) {
                        crate::md::writeln(&term, &l);
                    }
                    line.clear();
                    *cursor_col = 0;
                    crate::term::prompt(&term);
                    crate::term::follow_cursor();
                    return;
                }

                *recall = None;
                if !line.is_empty() {
                    crate::term::new_line(&term);
                    crate::shell::record(&line);
                    let line_match: &str = &line.trim();
                    match line_match {
                        "help" | "?" => crate::term::help_text(&term),
                        "about" => crate::term::about(&term),
                        "resume" => crate::term::subcommand_help_text("resume", "resume xp", &term),
                        "contact" => crate::term::contact_info(&term),
                        "clear" => crate::term::reset_window(&term),
                        "george" => crate::term::random_george_pic(&term),
                        "henry" => crate::term::random_henry_pic(&term),
                        "trout" => crate::term::random_trout_pic(&term),
                        "env" => crate::term::env_text(&term),
                        "replay" => {
                            let window = web_sys::window().unwrap();
                            local.set_intro_played(false);
                            local.save_to_local_storage();
                            window.location().reload().expect("could not reload");
                        },
                        _ => {
                            if crate::term::is_privesc(line_match) {
                                crate::term::throw_hackerman(&term);
                            } else if line_match.contains("books") || line_match.contains("features") {
                                crate::term::publications(&term, line_match);
                            } else if line_match.starts_with("resume ") {
                                crate::term::resume(&term, line_match);
                            } else if line_match.starts_with("goto ") {
                                crate::term::goto_links(&term, line_match);
                            } else if line_match == "top" || line_match == "htop" {
                                crate::fx::top(term.clone().unchecked_into());
                            } else if crate::fun::is_meltdown(line_match) {
                                crate::fx::meltdown(term.clone().unchecked_into());
                            } else if !crate::shell::run(&term, line_match) {
                                crate::shell::command_not_found(&term, line_match);
                            }
                        },
                    }

                    line.clear();
                    *cursor_col = 0;
                }

                if crate::fx::is_busy() {
                    // the animation prints the prompt when it's done
                } else if crate::shell::is_awaiting_password() {
                    term.write(crate::shell::password_prompt());
                } else {
                    crate::term::prompt(&term);
                }
            }
            crate::term::KEY_BACKSPACE => {
                if *cursor_col > 0 {
                    let at = byte_index(&line, *cursor_col - 1);
                    line.remove(at);
                    *cursor_col -= 1;
                    if !crate::shell::is_awaiting_password() {
                        // back up, redraw what was after the cursor, blank the last cell
                        let rest: String = line.chars().skip(*cursor_col).collect();
                        term.write(&format!("\u{0008}{} \x1b[{}D", rest, rest.chars().count() + 1));
                    }
                }
            }
            crate::term::KEY_LEFT_ARROW => {
                if *cursor_col > 0 && !crate::shell::is_awaiting_password() {
                    term.write(crate::term::CURSOR_LEFT);
                    *cursor_col -= 1;
                }
            }
            crate::term::KEY_RIGHT_ARROW => {
                if *cursor_col < line.chars().count() && !crate::shell::is_awaiting_password() {
                    term.write(crate::term::CURSOR_RIGHT);
                    *cursor_col += 1;
                }
            }
            crate::term::KEY_HOME if !crate::shell::is_awaiting_password() => {
                if *cursor_col > 0 {
                    term.write(&format!("\x1b[{}D", *cursor_col));
                    *cursor_col = 0;
                }
            }
            crate::term::KEY_END if !crate::shell::is_awaiting_password() => {
                let len = line.chars().count();
                if *cursor_col < len {
                    term.write(&format!("\x1b[{}C", len - *cursor_col));
                    *cursor_col = len;
                }
            }
            // up/down: walk back through what's been typed, like a real shell
            crate::term::KEY_UP_ARROW | crate::term::KEY_DOWN_ARROW if !crate::shell::is_awaiting_password() => {
                let typed = crate::shell::typed();
                if !typed.is_empty() {
                    let up = event.key_code() == crate::term::KEY_UP_ARROW;
                    *recall = match (*recall, up) {
                        (None, true) => Some(typed.len() - 1),
                        (Some(0), true) => Some(0),
                        (Some(i), true) => Some(i - 1),
                        (Some(i), false) if i + 1 < typed.len() => Some(i + 1),
                        (_, false) => None,
                    };
                    let shown = recall.map(|i| typed[i].clone()).unwrap_or_default();

                    // back to the start of the input, clear it, write the recalled line
                    if *cursor_col > 0 {
                        term.write(&format!("\x1b[{}D", *cursor_col));
                    }
                    term.write(&format!("\x1b[K{}", shown));
                    *cursor_col = shown.chars().count();
                    *line = shown;
                }
            }
            crate::term::KEY_L if event.ctrl_key() => term.writeln(""),
            crate::term::KEY_C if event.ctrl_key() => {
                if crate::shell::is_awaiting_password() {
                    crate::shell::submit_password("");
                }
                crate::term::prompt(&term);
                line.clear();
                *cursor_col = 0;
            }
            crate::term::KEY_U if event.ctrl_key() => {
                crate::term::prompt(&term);
                line.clear();
                *cursor_col = 0;
            }
            _ => {
                // only real characters: named keys (Tab, Escape, F1, ...) and
                // escape sequences don't belong in the line
                let key = e.key();
                let mut chars = key.chars();
                let printable = match (chars.next(), chars.next()) {
                    (Some(c), None) => !c.is_control(),
                    _ => false,
                };

                if printable && !event.alt_key() && !event.ctrl_key() && !event.meta_key() {
                    line.insert_str(byte_index(&line, *cursor_col), &key);
                    *cursor_col += 1;

                    // passwords aren't echoed
                    if !crate::shell::is_awaiting_password() {
                        // write the char plus whatever was after the cursor, then step back
                        let rest: String = line.chars().skip(*cursor_col).collect();
                        if rest.is_empty() {
                            term.write(&key);
                        } else {
                            term.write(&format!("{}{}\x1b[{}D", key, rest, rest.chars().count()));
                        }
                    }
                }
            }
        }
        crate::term::follow_cursor();
    }) as Box<dyn FnMut(_)>);

    terminal.on_key(callback.as_ref().unchecked_ref());
    callback.forget();

    // pasted text arrives through onData, not onKey. single characters (normal
    // typing) and escape sequences (arrows) are already handled above
    let paste_term: Terminal = terminal.clone().dyn_into()?;
    let paste = Closure::wrap(Box::new(move |data: String| {
        if data.chars().count() < 2 || data.starts_with('\x1b') {
            return;
        }
        // first line only, no control characters
        let text: String = data
            .split(['\r', '\n'])
            .next()
            .unwrap_or("")
            .chars()
            .filter(|c| !c.is_control())
            .collect();
        if text.is_empty() {
            return;
        }

        let mut editing = editor.borrow_mut();
        let (line, cursor_col, _) = &mut *editing;
        line.insert_str(byte_index(line, *cursor_col), &text);
        *cursor_col += text.chars().count();
        if !crate::shell::is_awaiting_password() {
            let rest: String = line.chars().skip(*cursor_col).collect();
            if rest.is_empty() {
                paste_term.write(&text);
            } else {
                paste_term.write(&format!("{}{}\x1b[{}D", text, rest, rest.chars().count()));
            }
        }
        crate::term::follow_cursor();
    }) as Box<dyn FnMut(String)>);
    terminal.on_data(paste.as_ref().unchecked_ref());
    paste.forget();
    terminal.load_addon(addon.clone().dyn_into::<FitAddon>()?.into());
    addon.fit();

    // refit whenever the window changes size (resizing, devtools, a phone's
    // keyboard opening), so the terminal never runs past the edge of the screen
    let refit = {
        let addon: FitAddon = addon.clone().unchecked_into();
        Closure::wrap(Box::new(move || addon.fit()) as Box<dyn FnMut()>)
    };
    web_sys::window().unwrap().add_event_listener_with_callback("resize", refit.as_ref().unchecked_ref())?;
    refit.forget();
    terminal.focus();

    run(cfg, intro_elem).await;

    Ok(())
}

// byte offset of the `n`th char in `s` (for editing at the cursor)
#[cfg(target_arch = "wasm32")]
fn byte_index(s: &str, n: usize) -> usize {
    s.char_indices().nth(n).map(|(i, _)| i).unwrap_or(s.len())
}
