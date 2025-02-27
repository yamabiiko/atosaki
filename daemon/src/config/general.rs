use crate::window::Window;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct General {
    pub apps: Vec<App>,
    pub cmds: Vec<Cmd>,
    pub terminal: Terminal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cmd {
    pub exe: String,
    pub save_cmd: String,
    pub restore_cmd: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub class: String,
    pub title: String,
    pub save_cmd: String,
    pub restore_cmd: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Terminal {
    pub class: String,
    pub exe: String,
    pub restore: String,
}

// for now, we don't want finegrained matching, but it should be configurable
impl General {
    pub fn replace_cmd(cmd: &str, win: &Window) -> String {
        cmd.replace("$$shell_id$$", &win.program.shell_id.to_string())
            .replace("$$pid$$", &win.program.pid.to_string())
    }
}

impl Terminal {
    pub fn is_match(&self, win: &Window) -> bool {
        let termc = Regex::new(&format!(r"{}", regex::escape(&self.class))).unwrap();
        termc.is_match(&win.class)
    }

    pub fn prepare_cli(&self, cmd: &str, win: &Window) -> String {
        self.restore.replace("$$cmd$$", &General::replace_cmd(cmd, win))
    }
}

impl Cmd {
    pub fn is_match(&self, win: &Window) -> bool {
        let rcmd = Regex::new(&format!(r"{}", regex::escape(&self.exe))).unwrap();
        rcmd.is_match(&win.program.cmdline)
    }
}

impl App {
    pub fn is_match(&self, win: &Window) -> bool {
        let appc = Regex::new(&format!(r"{}", regex::escape(&self.class))).unwrap();
        let appt = Regex::new(&format!(r"{}", regex::escape(&self.title))).unwrap();
        appc.is_match(&win.class) || appt.is_match(&win.title)
    }
}
