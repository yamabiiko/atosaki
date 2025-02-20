use crate::window::window::{Window, Program};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, Read};
use std::fs::OpenOptions;

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
        todo!()
    }

    pub fn load(&mut self, file: &str) -> std::io::Result<()> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        self.window_data = bincode::deserialize(&buffer).unwrap();
        todo!()
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
