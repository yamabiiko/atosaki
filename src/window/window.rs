//use std::process; // replace with nix ?
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Window {
    pub address: u64,
    pub at: [i32; 2],
    pub size: [i32; 2],
    pub monitor: u64,
    pub workspace: i32,
    //address: String,
    pub class: String,
    pub title: String,
    pub init_class: String,
    pub init_title: String,
    pub pinned: bool,
    pub fullscreen: bool,
    pub program: Program,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub shell_id: i32,
    pub pid: i32,
    pub cwd: String,
    pub exe: String,
    pub cmdline: String,
}
