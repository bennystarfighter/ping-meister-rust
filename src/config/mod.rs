use core::panic;
use serde::{self, Deserialize, Serialize};
use serde_yaml;
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub update_interval: u32,
    pub targets: Vec<Target>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Target {
    pub name: String,
    pub timeout: u32,
    pub address: String,
    pub r#type: String,
}

pub fn read_config(path: String) -> Config {
    let file_result = File::open(path);
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Failed to parse config file, {:}", error),
    };

    let mut config_content = String::new();
    let config_content_result = file.read_to_string(&mut config_content);
    match config_content_result {
        Ok(_) => (),
        Err(error) => panic!("{:}", error),
    }

    let yaml_result = serde_yaml::from_str(&config_content);
    match yaml_result {
        Ok(targets) => targets,
        Err(error) => panic!("{:}", error),
    }
}
