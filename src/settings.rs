use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
  pub full_url: String,
  pub hostname: String,
  pub port: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
  pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StaticPaths {
  pub dist: String,
  pub assets: String,
  pub templates: String,
  pub favicon: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ssl {
  pub key_file: String,
  pub cert_file: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
  pub server: Server,
  pub log: Log,
  pub static_paths: StaticPaths,
  pub ssl: Ssl,
  pub env: ENV,
}

const CONFIG_FILE_PATH: &str = "./config/Default.yml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "Dev".into());
    let mut s = Config::new();
    s.set("env", env.clone())?;

    s.merge(File::with_name(CONFIG_FILE_PATH))?;
    s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

    // and env var prefixed with 'KF_' will override any previously set value
    s.merge(Environment::with_prefix("kf").separator("_"))?;

    s.try_into()
  }
}

#[derive(Debug, Deserialize, Clone)]
pub enum ENV {
  Dev,
  Prod,
}

impl fmt::Display for ENV {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ENV::Dev => write!(f, "Dev"),
      ENV::Prod => write!(f, "Prod")
    }
  }
}

impl From<&str> for ENV {
  fn from(env: &str) -> Self {
    match env {
      "Prod" => ENV::Prod,
      _ => ENV::Dev,
    }
  }
}