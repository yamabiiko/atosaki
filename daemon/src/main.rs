use std::fs;
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
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{BufReader, AsyncReadExt};


const CONF: &str = "/home/yamabiko/.config/kuukiyomu/config.toml";
const SOCKET_LIST: &str = "/tmp/kuukiyomud";
const SAVE_FILE: &str = "/home/yamabiko/example";


#[tokio::main]
async fn main() {
    let cfg_str = fs::read_to_string(CONF)
        .expect("Failed to read file");
    let config: General = toml::from_str(&cfg_str)
        .expect("Could not parse configuration");
    println!("{:?}", config);
    let session = Arc::new(Mutex::new(Session::new()));


    let (sender, receiver) = mpsc::channel(32);
    let hyprland = Hyprland { session: session.clone(), sender: sender.clone() };
    tokio::spawn(  async move { hyprland.run(receiver).await } );

    let listener = UnixListener::bind(SOCKET_LIST).unwrap();
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn( handle_client(stream, session.clone(), sender.clone() ) );

    }

    todo!();

}

async fn handle_client(stream: UnixStream, session: Arc<Mutex<Session>>, sender: mpsc::Sender<SessionCmd>) -> () {
    let mut reader = BufReader::new(stream);
    let mut buffer = vec![0; 8];
    while let Ok(size) = reader.read(&mut buffer).await {
        if size > 0 {
            let req = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
            match ClientReq::from_u32(req) {
                Some(ClientReq::Save) => { let _  = session.lock().await.save(SAVE_FILE); () },
                Some(ClientReq::Load) => { let _ = session.lock().await.load(SAVE_FILE); let _ =sender.send(SessionCmd::Open).await; () },
                Some(ClientReq::Replace) => {  let _ = sender.send(SessionCmd::Close).await; let _ = session.lock().await.load(SAVE_FILE); 
                    let _ = sender.send(SessionCmd::Open).await; () },
                _ => println!("")
            }
        }
    }
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
