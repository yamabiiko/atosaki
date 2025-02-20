use crate::manager::{WindowManager, SessionCmd};
use crate::window::window::{Window, Program};
use crate::session::session::Session;

use std::{sync::Arc, path::Path, env};
use std::collections::HashMap;
use tokio::sync::{Mutex, mpsc};
use tokio::net::{UnixStream, UnixListener};
use tokio::io::{self, BufReader, AsyncReadExt, AsyncWriteExt};
use std::fs;
use std::path::PathBuf;


const SOCKET_PATH: &str = "/tmp/kuukiyomu_hypr.sock";
pub struct Hyprland {
    pub window_data: Arc<Mutex<Session>>,
    pub sender: mpsc::Sender<SessionCmd>,
}

#[repr(u32)] // Ensure the same representation as C++
#[derive(Debug)]
enum EventType {
    Add = 0,
    Update = 1,
    Delete = 2,
}

impl EventType {
    fn from_u32(value: u32) -> Option<EventType> {
        match value {
            0 => Some(EventType::Add),
            1 => Some(EventType::Update),
            2 => Some(EventType::Delete),
            _ => None,
        }
    }
}

impl Hyprland {
    async fn handle_client(stream: UnixStream, window_data: Arc<Mutex<Session>>) -> std::io::Result<()> {
        let (reader, _writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut buffer = vec![0; 1024];

        while let Ok(size) = reader.read(&mut buffer).await {
            if size > 0 {
                let req = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                let data = parse_data(&buffer[..size]);
                let mut wd = window_data.lock().await;
                match EventType::from_u32(req) {
                    Some(EventType::Add | EventType::Update) => {
                        wd.update_win(data);
                        println!("Updated");
                    },
                    Some(EventType::Delete) => { 
                        wd.delete_win(data.address);
                        println!("Deleted");
                    },
                    None => ()
                }
            }
        }
        Ok(())
    }

    pub async fn run(&self, mut rx: mpsc::Receiver<SessionCmd>) -> ()  {
        let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set");

        let hyprland_instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .expect("HYPRLAND_INSTANCE_SIGNATURE not set");

        let mut path = PathBuf::from(xdg_runtime_dir);
        path.push("hypr");
        path.push(&hyprland_instance_signature);
        path.push(".socket.sock");

        if Path::new(SOCKET_PATH).exists() {
            fs::remove_file(SOCKET_PATH).unwrap();
        }

        let listener = UnixListener::bind(SOCKET_PATH).unwrap();

        let mut hypr_sk = UnixStream::connect(Path::new(&path)).await.unwrap();

        loop {
            tokio::select! {
                Some(command) = rx.recv() => {
                    match command {
                        SessionCmd::Open(()) => {
                            let command = format!(
                                "dispatch exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo;] {}",
                                1, 100, 100, 100, 100, "alacritty" 
                            );
                            hypr_sk.write_all(command.as_bytes()).await.unwrap();
                        }
                        SessionCmd::Close(()) => {
                            todo!()
                        }
                    }
                },
                res = listener.accept() => {
                    let window_data = Arc::clone(&self.window_data);
                    let (stream, _) = res.unwrap();
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_client(stream, window_data).await {
                            eprintln!("Error handling client: {}", e);
                        }
                    });
                }
            }
        }
    }
}

impl WindowManager for Hyprland {
    async fn send_command(&self, command: SessionCmd) {
        if let Err(e) = self.sender.send(command).await {
            eprintln!("Failed to send command: {:?}", e);
        }
    }
}

fn parse_data(buffer: &[u8]) -> Window {
    use std::convert::TryInto;
    let mut offset = 4;
    let read_u64 = |offset: &mut usize| -> u64 {
        let val = u64::from_le_bytes(buffer[*offset..*offset+8].try_into().unwrap());
        *offset += 8;
        val
    };
    let read_i32 = |offset: &mut usize| -> i32 {
        let val = i32::from_le_bytes(buffer[*offset..*offset+4].try_into().unwrap());
        *offset += 4;
        val
    };

    let read_string = |offset: &mut usize| -> String {
        let len = u32::from_le_bytes(buffer[*offset..*offset+4].try_into().unwrap());
        *offset += 4;
        let str_bytes = &buffer[*offset..*offset + len as usize];
        *offset += len as usize;
        String::from_utf8_lossy(str_bytes).to_string()
    };

    Window {
        address: read_u64(&mut offset),
        at: [read_i32(&mut offset), read_i32(&mut offset)],
        size: [read_i32(&mut offset), read_i32(&mut offset)],
        monitor: read_u64(&mut offset),
        workspace: read_i32(&mut offset),
        class: read_string(&mut offset),
        title: read_string(&mut offset),
        init_class: read_string(&mut offset),
        init_title: read_string(&mut offset),
        pinned: buffer[offset] != 0,
        fullscreen: buffer[offset + 1] != 0,
        program: Program {
            pid: read_i32(&mut offset),
            shell_id: 0,
            cwd: String::new(),
            exe: String::new(),
            cmdline: String::new()
        }
    }
}
