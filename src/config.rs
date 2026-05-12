use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub protocol: String,
    pub timeout_ms: u64,
    pub ports: Vec<u16>,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_content)?;

    Ok(config)
}
