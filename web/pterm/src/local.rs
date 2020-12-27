use crate::utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename = "kathleenfrench", bound = "")]
pub struct Store {
  intro_played: bool,
}

impl Store {
  pub fn new() -> Self {
    Self::from_local_storage().unwrap_or_default()
  }

  pub fn from_local_storage() -> Option<Self> {
    utils::local_storage()
        .get("kathleenfrench")
        .ok()
        .and_then(|opt| opt)
        .and_then(|json| serde_json::from_str(&json).ok())
  }

  pub fn save_to_local_storage(&self) {
    let serialized = serde_json::to_string(self).unwrap_throw();
    utils::local_storage()
         .set("kathleenfrench", &serialized)
         .unwrap_throw();
  }

  pub fn set_intro_played(&mut self, played: bool) {
    self.intro_played = played;
  }

  pub fn get_intro_played(&mut self) -> bool {
    self.intro_played
  }
}