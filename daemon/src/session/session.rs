use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use tokio::process::Command;

use crate::window::{WinType, Window};

use crate::config::General;
use serde::{Deserialize, Serialize};

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
    pub async fn save(&self, file: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new().write(true).append(false).open(file)?;
        let encoded: Vec<u8> = bincode::serialize(&self.window_data).unwrap();

        let tasks: Vec<_> = self
            .window_data
            .iter()
            .filter_map(|(_, win)| match &win.wtype {
                WinType::CliApp(cmd) => Some(run_command(General::replace_cmd(&cmd.save_cmd, &win))),
                WinType::App(app) => Some(run_command(General::replace_cmd(&app.save_cmd, &win))),
                _ => None,
            })
            .collect();

        for t in tasks {
            let _ = t.await;
        }

        file.write_all(&encoded)?;
        Ok(())
    }

    pub async fn load(&mut self, file: &str) -> std::io::Result<()> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        self.window_data = bincode::deserialize(&buffer).unwrap();

        let _tasks: Vec<_> = self
            .window_data
            .iter_mut()
            .filter_map(|(_, win)| match &win.wtype {
                WinType::CliApp(cmd) => {
                    win.program.cmdline = self.config.terminal.prepare_cli(&cmd.restore_cmd, win);
                    None
                }
                WinType::App(app) => Some(General::replace_cmd(&app.restore_cmd, win)),

                WinType::Terminal => {
                    None
                }
                _ => None,
            })
            .collect();

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

async fn run_command(cmdline: String) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmdline)
        .stdout(std::process::Stdio::piped())
        .spawn()?
        .wait()
        .await?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Command `{}` failed with exit code {:?}",
            cmdline,
            status.code()
        )
        .into())
    }
}
