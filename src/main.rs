use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::env;
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};
use procfs::process::Process;

const ENV_HYPR_SIG: &str = "HYPRLAND_INSTANCE_SIGNATURE";
const ENV_WORKDIR: &str = "XDG_RUNTIME_DIR";


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
    pid: u16,
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

#[derive(Debug, PartialEq)]
enum Type {
    Shell,
    Nvim
}

#[derive(Debug)]
struct WindowData {
    at: [u16; 2],
    size: [u16; 2],
    class: Type,
    shell_id: i32,
    workspace: u8
}

fn main() {
    let hypr_sig = env::var(ENV_HYPR_SIG).unwrap();
    let workdir = env::var(ENV_WORKDIR).unwrap();
    let mut stream = UnixStream::connect(workdir.clone() + "/hypr/" + &hypr_sig + "/.socket.sock").unwrap();
    let mut response = String::new();
    stream.write_all(b"j/clients").unwrap();
    stream.read_to_string(&mut response).unwrap();
    let v: Vec<HyprWindowData> = serde_json::from_str(&response).unwrap();
    let mut saved: Vec<WindowData> = vec![];
    // let req = format!("dispatch exec [workspace 8 silent; float; size {}, {}; move {}, {}; pseudo] alacritty",
    for window in v.iter() {
        if window.class == "Alacritty" {
            let mut to_s = WindowData { at: window.at, size: window.size, shell_id: window.title.parse().unwrap(), class: Type::Shell, workspace: window.workspace.id };
            let shell = Process::new(to_s.shell_id).unwrap();
            let task = shell.task_from_tid(to_s.shell_id).unwrap();
            let children = task.children().unwrap();
            for c in children {
                let shell = Process::new(c.try_into().unwrap()).unwrap();
                if shell.cmdline().unwrap()[0] == "nvim" {
                    to_s.class = Type::Nvim;
                }
            }
            saved.push(to_s);
        }
    }
    for saved_w in saved.iter() {
        let cmd = if saved_w.class == Type::Shell { "~/Projects/alacritty/target/release/alacritty"} else { "~/Projects/alacritty/target/release/alacritty -e nvim" };
        let req = format!("dispatch exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo] {}",
            saved_w.workspace + 1, saved_w.size[0], saved_w.size[1], saved_w.at[0], saved_w.at[1], cmd);
        let mut stream = UnixStream::connect(workdir.clone() + "/hypr/" + &hypr_sig + "/.socket.sock").unwrap();
        stream.write_all(&req.into_bytes()).unwrap();
    }
    println!("{:?}", saved);
}
