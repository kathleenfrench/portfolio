use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StaticPaths {
    pub assets: String,
    pub templates: String,
    pub favicon: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
    pub static_paths: StaticPaths,
}

const CONFIG_FILE_PATH: &str = "./config/Default.yml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    // config/Default.yml, then config/<ENVIRONMENT>.yml (Dev or Prod), then any
    // env var prefixed with 'KF_' (e.g. KF_LOG_LEVEL=debug) overrides both
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "Dev".into());

        Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
            .add_source(Environment::with_prefix("kf").separator("_"))
            .build()?
            .try_deserialize()
    }
}
