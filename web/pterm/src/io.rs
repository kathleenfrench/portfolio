use wasm_bindgen::prelude::*;

use crate::SPEED_FACTOR;

#[cfg(not(target_arch = "wasm32"))]
pub async fn csleep(length: u64) {
  use std::time;
  let speed_factor = *SPEED_FACTOR.lock().await;
  let sleep_length = time::Duration::from_millis((1.0 / speed_factor * length as f32) as u64);
  async_std::task::sleep(sleep_length).await;
}

#[cfg(target_arch = "wasm32")]
pub async fn csleep(length: u64) {
  let speed_factor = *SPEED_FACTOR.lock().await;
  let sleep_length = (1.0 / speed_factor * length as f32) as i32;

  let promise = js_sys::Promise::new(&mut move |resolve, _| {
    let window = web_sys::window().expect("should have a global Window object");
    window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, sleep_length).expect("do not expect error on setTimeout()");
  });

  let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

#[wasm_bindgen(inline_js = "export function pterm_write(s) { window.pterm.write(s) }")]
extern "C" {
  pub fn pterm_write(s: &str);
}

pub async fn delayed_print<S: Into<String>>(s: S, delay: u64) {
  let input_arr = s.into().chars().map(|x| x.to_string()).collect::<Vec<String>>();

  for i in input_arr {
    #[cfg(target_arch = "wasm32")] 
    {
      pterm_write(&i)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
      use std::io::stdout;
      use std::io::Write;

      print!("{}", i);
      stdout().flush().unwrap();
    }

    if delay > 0 {
      csleep(delay).await;
    }
  }
}

pub async fn print<S: Into<String>>(s: S) {
  delayed_print(s, 0).await;
}

pub async fn new_line() {
  print("\r\n").await;
}

pub async fn clear_line() {
  print("\x1b[H\x1b[2J").await;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_terminal_width() -> usize {
  term_size::dimensions().expect("not attached to the terminal").0
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = "export function get_terminal_width() { return window.pterm.cols }")]
extern "C" {
  pub fn get_terminal_width() -> usize;
}