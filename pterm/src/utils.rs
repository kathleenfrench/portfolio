use rand::distributions::Uniform;
use rand::prelude::*;
use rand_distr::{ChiSquared, Exp};
use std::cmp;
use std::path::{Path, PathBuf};
use std::str;

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