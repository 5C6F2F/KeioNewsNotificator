use crate::line::NotifyOnLine;
use std::fs;

static CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub line: NotifyOnLine,
}

impl Config {
    pub fn new() -> Self {
        let config_str = fs::read_to_string(CONFIG_PATH).expect("Failed to read config.toml");
        let config: Config = toml::from_str(&config_str).expect("Failed to parse config.toml");

        config
    }
}
