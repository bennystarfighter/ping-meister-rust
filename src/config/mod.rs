use serde::{self, Deserialize, Serialize};
use serde_yaml;
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub update_interval: u32,
    pub targets: Vec<Target>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Target {
    pub name: String,
    pub timeout: u32,
    pub address: String,
    pub r#type: String,
}

#[derive(Debug)]
pub enum ConfigError {
    FileError(std::io::Error),
    DecodeError(serde_yaml::Error),
}

pub fn read_config(path: String) -> Result<Config, ConfigError> {
    let mut file = File::open(path).map_err(ConfigError::FileError)?;

    let mut config_content = String::new();
    file.read_to_string(&mut config_content)
        .map_err(ConfigError::FileError)?;

    serde_yaml::from_str(&config_content).map_err(ConfigError::DecodeError)
}
