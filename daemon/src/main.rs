use anyhow::{anyhow, Context};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;
use toml;

mod config;
mod hyprland;
mod manager;
mod session;
mod window;

use crate::config::General;
use crate::hyprland::hyprland::Hyprland;
use crate::manager::WindowManager;
use crate::session::session::Session;


const CONF: &str = "/home/yamabiko/.config/atosaki/config.toml";
const SOCKET_LIST: &str = "/tmp/atosakid";
const SAVE_FILE: &str = "/home/yamabiko/example";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg_str = fs::read_to_string(CONF).context(format!("Failed to read the file {}", CONF))?;

    let config: General = toml::from_str(&cfg_str)
        .context(format!("Could not parse the configuration at {}", CONF))?;

    let session = Arc::new(Mutex::new(Session::new(config, Hyprland {})));

    if Path::new(SOCKET_LIST).exists() {
        fs::remove_file(SOCKET_LIST).context(format!(
            "Failed to remove stale Unix socket file {}",
            SOCKET_LIST
        ))?;
    }

    let listener = UnixListener::bind(SOCKET_LIST)
        .context(format!("Failed to bind Unix socket {}", SOCKET_LIST))?;

    loop {
        tokio::select! {
            Ok((stream, _)) = listener.accept() => {
                let session_clone = Arc::clone(&session);
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, session_clone).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
        }
    }
}

async fn handle_client<T: WindowManager>(
    stream: UnixStream,
    session: Arc<Mutex<Session<T>>>,
) -> anyhow::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut buffer = vec![0; 8];
    while let Ok(size) = reader.read(&mut buffer).await {
        let mut session = session.lock().await;
        if size > 0 {
            let req = u32::from_le_bytes(
                buffer[0..4]
                    .try_into()
                    .context(format!("Could not read client request type"))?,
            );
            match ClientReq::from_u32(req) {
                Some(ClientReq::Save) => {
                    println!("Trying to save");
                    session.save(SAVE_FILE).await?;
                    ()
                }
                Some(ClientReq::Load) => {
                    println!("Trying to load");
                    session.load(SAVE_FILE).await?;
                    ()
                }
                Some(ClientReq::Replace) => {
                    session.replace(SAVE_FILE).await?;
                    ()
                }
                _ => return Err(anyhow!("Unknown client request type")),
            }
        } else {
            break;
        }
    }
    Ok(())
}

#[repr(u32)]
#[derive(Debug)]
enum ClientReq {
    Save = 0,
    Load = 1,
    Replace = 2,
}

impl ClientReq {
    fn from_u32(value: u32) -> Option<ClientReq> {
        match value {
            0 => Some(ClientReq::Save),
            1 => Some(ClientReq::Load),
            2 => Some(ClientReq::Replace),
            _ => None,
        }
    }
}
