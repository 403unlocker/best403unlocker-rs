use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub file_url: String,
    pub dns: Vec<String>,
    pub timeout: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("./best403unlocker.conf");
        let config_content = if config_path.exists() {
            fs::read_to_string(config_path)?
        } else {
            let etc_config_path = Path::new("/etc/best403unlocker.conf");
            if etc_config_path.exists() {
                fs::read_to_string(etc_config_path)?
            } else {
                todo!("Implement downloading the config file if not found");
                // TODO: Implement downloading the config file if not found
                return Err("Configuration file not found".into());
            }
        };
        let mut config: Config = toml::from_str(&config_content)?;
        config.dns = config.dns.iter().map(|s| s.trim().to_string()).collect();
        Ok(config)
    }
}
