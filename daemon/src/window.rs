use hyprland::data::Client;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::config::general::{App as CApp, Cli};
use crate::config::General;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowRegistry {
    data: BTreeMap<String, Window>,
    config: General,
}

impl WindowRegistry {
    pub fn new(config: General) -> Self {
        Self {
            data: BTreeMap::new(),
            config,
        }
    }

    pub fn update(&mut self, data: Vec<Window>) -> () {
        self.data = data.into_iter().map(|w| (w.address.clone(), w)).collect();
    }

    pub fn on_save(&self) -> Vec<String> {
        self.data
            .iter()
            .filter_map(|(_, win)| match &win.wtype {
                WinType::CliApp(Some(cmd)) => cmd
                    .on_save
                    .as_ref()
                    .map(|save| General::expand_vars(save, &win)),
                WinType::App(app) => app
                    .on_save
                    .as_ref()
                    .map(|save| General::expand_vars(save, &win)),
                _ => None,
            })
            .collect()
    }

    pub fn difference(&self, other: &Self) -> Self {
        WindowRegistry {
            data: self
                .data
                .iter()
                .filter(|(key, _)| !other.data.contains_key(*key))
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
            config: self.config.clone(),
        }
    }

    pub fn win_vec(&self) -> Vec<&Window> {
        self.data.iter().map(|(_, win)| win).collect()
    }

    // should we update, set cmdline and program type all in one go? probably
    pub fn set_cmdline(&mut self) {
        for (_, win) in self.data.iter_mut() {
            match &win.wtype {
                WinType::CliApp(cli) => {
                    let exec_cmd = cli
                        .as_ref()
                        .and_then(|c| c.exec.as_ref())
                        .unwrap_or(&win.program.cmdline);

                    win.program.cmdline = self.config.terminal.prepare_cli(exec_cmd, &win);
                }
                WinType::App(app) => {
                    win.program.cmdline = app.exec.clone().unwrap_or(win.program.cmdline.clone());
                }
                _ => (),
            }
        }
    }

    pub fn set_program_type(&mut self) {
        for (_, win) in self.data.iter_mut() {
            if self.config.terminal.is_match(win) {
                win.wtype = WinType::Terminal;
                win.program.cmdline = self.config.terminal.bin.clone();

                let shell_id = get_child_pids(win.program.pid);
                if let Some(shell_id) = shell_id.first() {
                    win.program.shell_id = *shell_id;
                    let cpid = get_child_pids(*shell_id);
                    if let Some(cpid) = cpid.first() {
                        win.program.cmdline = get_cmdline(*cpid).unwrap();
                        win.program.pid = *cpid;
                        let cli_app = self
                            .config
                            .cli
                            .iter()
                            .find(|cmd| cmd.is_match(&win))
                            .map(|cmd| cmd.clone());
                        win.wtype = WinType::CliApp(cli_app);
                    }
                }
            } else {
                win.program.cmdline = get_cmdline(win.program.pid).unwrap();
                if let Some(app) = self.config.app.iter().find(|app| app.is_match(&win)) {
                    win.wtype = WinType::App(app.clone());
                } else {
                    win.wtype = WinType::Plain
                }
            }
        }
    }
}

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
    pub floating: bool,
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
    CliApp(Option<Cli>),
    App(CApp),
    Plain,
}

impl Ord for Window {
    fn cmp(&self, other: &Self) -> Ordering {
        self.address.cmp(&other.address)
    }
}
impl PartialOrd for Window {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.address.cmp(&other.address))
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Eq for Window {}

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
            floating: client.floating,
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
