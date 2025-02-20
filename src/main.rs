use std::fs;
use std::collections::HashMap;
use toml;

mod config;
mod manager;
mod hyprland;
mod window;
mod session;
use config::general::General;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::session::session::Session;
use crate::hyprland::hyprland::Hyprland;
use crate::manager::SessionCmd;

use tokio::time::{sleep, Duration};

const CONF: &str = "/home/yamabiko/.config/kuukiyomu/config.toml";

#[tokio::main]
async fn main() {
    let cfg_str = fs::read_to_string(CONF)
        .expect("Failed to read file");
    let config: General = toml::from_str(&cfg_str)
        .expect("Could not parse configuration");
    println!("{:?}", config);
    let session = Arc::new(Mutex::new(Session::new()));


    let (sender, receiver) = mpsc::channel(32);
    let hyprland = Hyprland { window_data: session.clone(), sender: sender.clone() };
    tokio::spawn(  async move { hyprland.run(receiver).await } );
    todo!();

}
