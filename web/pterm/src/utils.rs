use rand::prelude::*;
use std::path::Path;
use std::str;

use web_sys::window;
use wasm_bindgen::UnwrapThrowExt;

pub static mut IS_MOBILE: bool = false;

pub fn gen_file_name_with_extension(rng: &mut ThreadRng, files: &[&str], extension: &str) -> String {
  let chosen_file = files.choose(rng).unwrap_or(&"");
  let path = Path::new(&chosen_file).with_extension(extension);
  path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn open_in_new_tab(url: &str) -> () {
  let window = window().unwrap();
  window.open_with_url_and_target(url, "_blank").unwrap();
}

pub fn local_storage() -> web_sys::Storage {
  let window = window().unwrap();
  window.local_storage().unwrap_throw().unwrap_throw()
}

pub fn is_mobile() -> bool {
  unsafe { IS_MOBILE }
}

pub fn desktop_or_mobile() {
  let nav = web_sys::window().unwrap().navigator();

  let mobile = [
    "Android",
    "BlackBerry",
    "IEMobile",
    "iPad",
    "iPhone",
    "iPod",
    "webOS",
    "Windows Phone",
  ];

  match nav.user_agent(){
    Ok(agent) => {
      for p in mobile.iter() {
        let is_match = agent
          .to_lowercase()
          .matches(p.to_lowercase().as_str())
          .next()
          .is_some();

        if is_match {
          unsafe {
            IS_MOBILE = true;
            break;
          }
        }
      }
    }
    Err(_) => panic!("could not determine browser type"),
  }
}