use crate::window::Window;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct General {
    pub app: Vec<App>,
    pub cli: Vec<Cli>,
    pub terminal: Terminal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cli {
    pub match_exe: String,
    pub exec: Option<String>,
    pub on_save: Option<String>,
    pub on_restore: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub class: String,
    pub title: String,
    pub exec: Option<String>,
    pub on_save: Option<String>,
    pub on_restore: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Terminal {
    pub class: String,
    pub bin: String,
    pub restore_cmd: String,
}

// for now, we don't want finegrained matching, but it should be configurable
impl General {
    pub fn expand_vars(cmd: &str, win: &Window) -> String {
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
        self.restore_cmd
            .replace("$$cmd$$", &General::expand_vars(cmd, win))
    }
}

impl Cli {
    pub fn is_match(&self, win: &Window) -> bool {
        let rcmd = Regex::new(&format!(r"{}", regex::escape(&self.match_exe))).unwrap();
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
