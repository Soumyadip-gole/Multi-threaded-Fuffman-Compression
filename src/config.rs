use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub encode_input_dir: PathBuf,
    pub encoded_output_dir: PathBuf,
    pub decode_input_dir: PathBuf,
    pub decoded_output_dir: PathBuf,
}

pub fn load() -> Config {
    let config_contents =
        fs::read_to_string("config.toml").expect("Failed to read config.toml file");
    toml::from_str(&config_contents).expect("Failed to parse config.toml file")
}
