use serde::{Deserialize, Serialize};
use hyprland::data::Client;

use crate::config::general::{App as CApp, Cmd};
use crate::config::General;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub address: String,
    pub at: (i16, i16),
    pub size: (i16, i16),
    pub monitor: i128,
    pub workspace: i32,
    pub class: String,
    pub title: String,
    pub init_class: String,
    pub init_title: String,
    pub pinned: bool,
    pub fullscreen: u8,
    pub program: Program,
    pub wtype: WinType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub shell_id: i32,
    pub pid: i32,
    pub cwd: String,
    pub exe: String,
    pub cmdline: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WinType {
    Terminal,
    CliApp(Option<Cmd>),
    App(CApp),
    Plain,
}

impl Window {
    pub fn set_program_type(&mut self, config: &General) -> () {
        if config.terminal.is_match(&self) {
            // get pid of nested program
            // this should search between all child pids and match accordigly

            self.wtype = WinType::Terminal;
            self.program.cmdline = config.terminal.exe.clone();
            let shell_id = get_child_pids(self.program.pid);
            if let Some(shell_id) = shell_id.first() {
                self.program.shell_id = *shell_id;
                let cpid = get_child_pids(*shell_id);
                if let Some(cpid) = cpid.first() {
                    self.program.cmdline = get_cmdline(*cpid).unwrap();
                    self.program.pid = *cpid;
                    let cli_app = config.cmds.iter().find(|cmd| cmd.is_match(&self)).map(|cmd| cmd.clone());
                    self.wtype = WinType::CliApp(cli_app);
                }
            }
        } else {
            self.program.cmdline = get_cmdline(self.program.pid).unwrap();
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
        .map(|s| {
            s.split_whitespace()
                .filter_map(|p| p.parse().ok())
                .collect()
        })
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

impl From<Client> for Window {
    fn from(client: Client) -> Self {
        Window {
            address: client.address.to_string(),
            at: client.at,
            size: client.size,
            monitor: client.monitor,
            workspace: client.workspace.id,
            class: client.class,
            title: client.title,
            init_title: client.initial_title,
            init_class: client.initial_class,
            pinned: client.pinned,
            fullscreen: 0, //client.fullscreen,
            program: Program {
                shell_id: i32::default(),
                pid: client.pid,
                cwd: String::default(),
                exe: String::default(),
                cmdline: String::default(),
            },
            wtype: WinType::Plain,
        }
    }
}
