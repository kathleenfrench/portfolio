use rand::distributions::Uniform;
use rand::prelude::*;
use rand_distr::{ChiSquared, Exp};
use std::cmp;
use std::path::{Path, PathBuf};
use std::str;

use web_sys::{window, Document, Element, HtmlElement, Window, Location};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn gen_file_name_with_extension(rng: &mut ThreadRng, files: &[&str], extension: &str) -> String {
  let chosen_file = files.choose(rng).unwrap_or(&"");
  let path = Path::new(&chosen_file).with_extension(extension);
  path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn gen_file_name(rng: &mut ThreadRng, files: &[&str], extensions: &[&str]) -> String {
  let chosen_file = files.choose(rng).unwrap_or(&"");
  let chosen_extension = extensions.choose(rng).unwrap_or(&"");
  let path = Path::new(&chosen_file).with_extension(chosen_extension);
  path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn set_location(url: &str) -> () {
  let window = window().unwrap();
  window.location().set_href(url).unwrap();
}

pub fn open_in_new_tab(url: &str) -> () {
  let window = window().unwrap();
  window.open_with_url_and_target(url, "_blank").unwrap();
}

pub fn window() -> web_sys::Window {
  web_sys::window().expect("no global 'window' exists")
}

pub fn document() -> web_sys::Document {
  window().document().expect("window should have a document")
}

pub fn body() -> web_sys::HtmlElement {
  document().body().expect("document should have a body")
}