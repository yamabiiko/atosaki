use std::fs;
use toml;

mod config;
mod manager;
mod hyprland;
mod window;
use config::general::General;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::window::window::Window;
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
    let w_data: Vec<Window> = Vec::new();

    let window_data = Arc::new(Mutex::new(w_data));


    let (sender, receiver) = mpsc::channel(32);
    let hyprland = Hyprland { window_data: window_data.clone(), sender: sender.clone() };
    tokio::spawn(  async move { hyprland.run(receiver).await } );
    loop { sender.send(SessionCmd::Open(())).await.unwrap(); sleep(Duration::from_secs(1)).await; }
}
