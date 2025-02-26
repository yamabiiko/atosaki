//use std::process; // replace with nix ?
use serde::{Serialize, Deserialize};

use crate::config::General;
use crate::config::general::{App as CApp, Cmd};

#[derive(Deserialize, Serialize, Debug)]
pub struct Window {
    pub address: u64,
    pub at: [i32; 2],
    pub size: [i32; 2],
    pub monitor: u64,
    pub workspace: i32,
    pub class: String,
    pub title: String,
    pub init_class: String,
    pub init_title: String,
    pub pinned: bool,
    pub fullscreen: bool,
    pub program: Program,
    pub wtype: WinType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub shell_id: i32,
    pub pid: i32,
    pub cwd: String,
    pub exe: String,
    pub cmdline: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WinType {
    Terminal,
    CliApp(Cmd),
    App(CApp),
    Plain
}

impl Window {
    pub fn set_program_type(&mut self, config: &General) -> () {
        if config.terminal.is_match(&self) {
            // get pid of nested program
            // this should search between all child pids and match accordigly

            self.wtype = WinType::Terminal;
            self.program.cmdline = config.terminal.exe.clone();
            let pids = get_child_pids(self.program.pid);
            if let Some(cpdi) = pids.first() {
                self.program.cmdline = get_cmdline(*cpdi).unwrap();
                self.program.shell_id = self.program.pid;
                self.program.pid = *cpdi;
                if let Some(cmd) = config.cmds.iter().find(|cmd| cmd.is_match(&self)) {
                    self.wtype = WinType::CliApp(cmd.clone());
                } else {
                    self.wtype = WinType::Terminal
                }
            }
        } else {
            if let Some(app) = config.apps.iter().find(|app| app.is_match(&self)) {
                self.wtype = WinType::App(app.clone());
            } else {
                self.wtype = WinType::Plain
            }
        }
    }
}

fn get_child_pids(pid: i32) -> Vec<i32> {
    let path = format!("/proc/{}/task/{}/children", pid, pid);
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.split_whitespace().filter_map(|p| p.parse().ok()).collect())
        .unwrap_or_default()
}

fn get_cmdline(pid: i32) -> Option<String> {
    let path = format!("/proc/{}/cmdline", pid);
    match std::fs::read_to_string(&path) {
        Ok(contents) => {
            let cmdline = contents.replace('\0', " ").trim().to_string();
            if cmdline.is_empty() {
                None
            } else {
                Some(cmdline)
            }
        }
        Err(_) => None,
    }
}
