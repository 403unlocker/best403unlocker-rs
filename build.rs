use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
const CONFIG_DIR: &str = "AppData/Roaming/best403unlocker";

#[cfg(not(target_os = "windows"))]
const CONFIG_DIR: &str = ".config/best403unlocker";

fn main() {
    // Path to the configuration file
    let conf_file = "best403unlocker.conf";

    // Home directory of the current user
    let home_dir = if cfg!(target_os = "windows") {
        env::var("USERPROFILE").expect("Couldn't read home directory path")
    } else {
        env::var("HOME").expect("Couldn't read home directory path")
    };

    // Path to the config directory
    let config_dir = PathBuf::from(&home_dir).join(CONFIG_DIR);

    // Create the config directory if it doesn't exist
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    // Copy the configuration file to the config directory
    let dest_path = config_dir.join(conf_file);
    println!("Copying configuration file to: {:?}", dest_path);
    fs::copy(conf_file, &dest_path).expect("Failed to copy configuration file");

    println!("cargo:rerun-if-changed={}", conf_file);
}