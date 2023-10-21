use std::process;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub monitor_interface: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: String::from("localhost"),
            port: 8000,
            monitor_interface: String::from("wifimon")
        }
    }
}

pub fn load_config(file: &str) -> Config {
    match confy::load_path(file) {
        Ok(config) => config,
        Err(error) => {
            error!("{}", error);
            process::exit(5);
        }
    }
}