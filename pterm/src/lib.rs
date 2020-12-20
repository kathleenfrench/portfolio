pub mod app;

mod io;
mod content;
mod sections;

pub static PORTFOLIO_SECTIONS: &[&str] = &[
    "intro",
    // "resume",
    // "about",
];

use futures::lock::Mutex;
use instant::Instant;
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32};

use app::AppConfig;
use rand::thread_rng;

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

pub async fn run(cfg: AppConfig) {
    let mut thread_range = thread_rng();

    loop {
        let choice: &str = cfg.sections.choose(&mut thread_range).unwrap();

        match choice {
            "intro" => sections::intro::run(&cfg).await,
            // "resume",
            // "about",
            _ => panic!("unknown section '{}'!", choice),
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

    run(cfg).await;
    Ok(())
}