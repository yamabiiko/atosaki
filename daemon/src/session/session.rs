use crate::window::window::{Window, Program};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};

#[derive(Debug)]
pub struct Session {
    pub window_data: HashMap<u64, Window>
}


impl Session {
    pub fn new() -> Self {
        Self {
            window_data: HashMap::new(),
        }
    }
    pub fn save(&self, file: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file)?;
        let encoded: Vec<u8> = bincode::serialize(&self.window_data).unwrap();

        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load(&mut self, file: &str) -> std::io::Result<()> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        self.window_data = bincode::deserialize(&buffer).unwrap();
        Ok(())
    }

    pub fn update_win(&mut self, window: Window) -> bool {
        self.window_data.insert(window.address, window);

        true
    }

    pub fn delete_win(&mut self, addr: u64) -> bool {
        self.window_data.remove_entry(&addr);

        true
    }
}

fn get_child_pids(pid: u32) -> Vec<u32> {
    let path = format!("/proc/{}/task/{}/children", pid, pid);
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.split_whitespace().filter_map(|p| p.parse().ok()).collect())
        .unwrap_or_default()
}

fn get_cmdline(pid: u32) -> Option<String> {
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
