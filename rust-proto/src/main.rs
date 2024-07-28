use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};
use procfs::process::Process;
use bincode;
use clap::Parser;

const ENV_HYPR_SIG: &str = "HYPRLAND_INSTANCE_SIGNATURE";
const ENV_WORKDIR: &str = "XDG_RUNTIME_DIR";

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
enum Commands {
    Save,
    Load,
    //Switch,
    //Daemon
}

#[derive(Serialize, Deserialize)]
struct Workspace {
    id: u8,
    name: String
}

#[derive(Serialize, Deserialize)]
struct HyprWindowData {
    address: String,
    mapped: bool,
    hidden: bool,
    at: [u16; 2],
    size: [u16; 2],
    workspace: Workspace,
    floating: bool,
    pseudo: bool,
    monitor: u8,
    class: String,
    title: String,
    initialClass: String,
    initialTitle: String,
    pid: i32,
    xwayland: bool,
    pinned: bool,
    fullscreen: bool,
    fullscreenMode: u8,
    fakeFullscreen: bool,
    grouped: Vec<u8>,
    tags: Vec<u8>,
    swallowing: String,
    focusHistoryID: u8

}

#[derive(Serialize, Deserialize, Debug)]
struct MsgPackRequest {
    r#type: u8,
    msgid: u32,
    method: String,
    params: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Type {
    Shell,
    Nvim
}


#[derive(Debug, Serialize, Deserialize)]
struct WindowData {
    at: [u16; 2],
    size: [u16; 2],
    class: Type,
    shell_id: i32,
    workspace: u8,
    cwd: PathBuf
}

const NVIM_CACHE_DIR: &str = "/home/yamabiko/.cache/nvim/session/";

fn main() {
    let hypr_sig = env::var(ENV_HYPR_SIG).unwrap();
    let workdir = env::var(ENV_WORKDIR).unwrap();
    let mut saved: Vec<WindowData> = vec![];
    let args = Commands::parse();
    match args {
        Commands::Save => {
            let mut stream = UnixStream::connect(workdir.clone() + "/hypr/" + &hypr_sig + "/.socket.sock").unwrap();
            let mut response = String::new();
            stream.write_all(b"j/clients").unwrap();
            stream.read_to_string(&mut response).unwrap();
            let v: Vec<HyprWindowData> = serde_json::from_str(&response).unwrap();

            for window in v.iter() {
                if window.class == "Alacritty" {
                    let mut to_s = WindowData { at: window.at, size: window.size, shell_id: window.title.parse().unwrap(), class: Type::Shell, workspace: window.workspace.id, cwd: PathBuf::new() };
                    let shell = Process::new(to_s.shell_id).unwrap();
                    to_s.cwd = shell.cwd().unwrap();
                    let task = shell.task_from_tid(to_s.shell_id).unwrap();
                    let children = task.children().unwrap();
                    for c in children {
                        let shell = Process::new(c.try_into().unwrap()).unwrap();
                        if shell.cmdline().unwrap()[0] == "nvim" {
                            to_s.class = Type::Nvim;

                            let addr = String::new() + NVIM_CACHE_DIR + &to_s.shell_id.to_string() + ".socket";
                            println!("{}", addr);
                            let mut nvim = UnixStream::connect(addr).unwrap();
                            let saving_addr = String::new() + NVIM_CACHE_DIR + &to_s.shell_id.to_string() + ".cache";
                            let save_cmd = "mksession! ".to_string() + &saving_addr;
                            let request = MsgPackRequest {
                                r#type: 0, // Request type
                                msgid: 1, // Sequence number, this can be incremented for each request
                                method: "nvim_command".to_string(),
                                params: vec![save_cmd],
                            };
                            let msgpack_data = rmp_serde::to_vec(&request).unwrap();

                            nvim.write(&msgpack_data).unwrap();
                            nvim.flush().unwrap();
                            let mut buffer: [u8; 1024] = [0; 1024];
                            let n = nvim.read(&mut buffer).unwrap();
                            println!("{}", n);
                        }
                    }
                    saved.push(to_s);
                }
            }
            let mut save_file = File::create(".session").unwrap();
            let encoded: Vec<u8> = bincode::serialize(&saved).unwrap();
            save_file.write_all(&encoded).unwrap();
            println!("{:?}", saved);
        }
        Commands::Load => {
            let mut save_file = File::open(".session").unwrap();
            let mut buffer: Vec<u8> = Vec::new();
            save_file.read_to_end(&mut buffer).unwrap();
            let decoded: Vec<WindowData> = bincode::deserialize(&buffer).unwrap();
            for saved_w in decoded.iter() {
                let cmd = match saved_w.class {
                    Type::Nvim => {
                        let cache_file = String::new() + NVIM_CACHE_DIR + &saved_w.shell_id.to_string() + ".cache";
                        let restore_cmd = " -S ".to_string() + &cache_file + "; bash -i'";
                        "~/Projects/alacritty/target/release/alacritty --working-directory ".to_string() + saved_w.cwd.to_str().unwrap() +  " -e bash -c 'nvim " + &restore_cmd
                    }
                    Type::Shell => {
                        "~/Projects/alacritty/target/release/alacritty --working-directory ".to_string() + saved_w.cwd.to_str().unwrap()
                    }
                };

                let mut stream = UnixStream::connect(workdir.clone() + "/hypr/" + &hypr_sig + "/.socket.sock").unwrap();
                let req = format!("dispatch exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo;] {}",
                    saved_w.workspace + 1, saved_w.size[0], saved_w.size[1], saved_w.at[0], saved_w.at[1], cmd);
                stream.write_all(&req.into_bytes()).unwrap();
            }
        }
    }
}
