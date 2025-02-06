use std::fs;
use toml;

mod config;
use config::general::General;

const CONF: &str = "/home/yamabiko/.config/kuukiyomu/config.toml";

fn main() {
    let cfg_str = fs::read_to_string(CONF)
        .expect("Failed to read file");
    let config: General = toml::from_str(&cfg_str)
        .expect("Could not parse configuration");
    println!("{:?}", config);
}
