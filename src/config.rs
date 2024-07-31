use serde::Deserialize;
use std::{env, fs, process};
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub file_url: String,
    pub dns: Vec<String>,
    pub timeout: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        //check different location for .conf file existence
        let config_path = Path::new("../best403unlocker.conf");
        let config_content = if config_path.exists() {
            fs::read_to_string(config_path)?
        }
            else {
               let config_path =if cfg!(target_os="windows"){

                   let home_dir = env::var("USERPROFILE").expect("Couldn't read home directory path");
                   let config_path = Path::new("AppData/Roaming/best403unlocker/best403unlocker.conf");
                   PathBuf::from(&home_dir).join(&config_path)

               }
                else{
                    let home_dir = env::var("HOME").expect("Couldn't read home directory path");
                    let config_path = Path::new(".config/best403unlocker/best403unlocker.conf");
                    PathBuf::from(&home_dir).join(&config_path)
                };
                if config_path.exists(){

                    fs::read_to_string(&config_path)?
                }

                else{
                    if cfg!(target_os = "windows"){
                    eprintln!("failed to find `best403unlocker.conf` file");
                        process::exit(1)
                    }
                    let etc_config_path = Path::new("/etc/best403unlocker.conf");
                    if etc_config_path.exists() {
                        fs::read_to_string(etc_config_path)?
                    } else {
                        eprintln!("failed to find `best403unlocker.conf` file");
                        process::exit(1);
                        todo!("Implement downloading the config file if not found");
                        // TODO: Implement downloading the config file if not found
                        return Err("Configuration file not found".into());
                    }
                }
            };
        let mut config: Config = toml::from_str(&config_content)?;
        config.dns = config.dns.iter().map(|s| s.trim().to_string()).collect();
        Ok(config)
    }
}
