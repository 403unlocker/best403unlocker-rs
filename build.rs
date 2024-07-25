use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
const BIN_DIR: &str = "bin";

#[cfg(not(target_os = "windows"))]
const BIN_DIR: &str = ".cargo/bin";

fn main() {
    // Path to the configuration file
    let conf_file = "best403unlocker.conf";

    // Home directory of the current user
    let home_dir = env::var("HOME").expect("Failed to get home directory");

    // Path to the ~/.cargo/bin/ directory
    let cargo_bin_dir = PathBuf::from(home_dir).join(BIN_DIR);

    // Copy the configuration file to the ~/.cargo/bin/ directory
    let dest_path = cargo_bin_dir.join(conf_file);
    println!("{:?}",dest_path);
    fs::copy(conf_file, &dest_path).expect("Failed to copy configuration file");

    println!("cargo:rerun-if-changed={}", conf_file);
}
