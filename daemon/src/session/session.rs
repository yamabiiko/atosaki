use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};

use crate::window::{Window, WinType};

use serde::{Serialize, Deserialize};
use crate::config::General;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub window_data: HashMap<u64, Window>,
    config: General,
}


impl Session {
    pub fn new(config: General) -> Self {
        Self {
            window_data: HashMap::new(),
            config,
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

    pub fn update_win(&mut self, mut window: Window) -> bool {
        window.set_program_type(&self.config);
        self.window_data.insert(window.address, window);

        true
    }

    pub fn delete_win(&mut self, addr: u64) -> bool {
        self.window_data.remove_entry(&addr);

        true
    }
}
