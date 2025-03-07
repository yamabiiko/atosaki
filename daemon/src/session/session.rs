use anyhow::{anyhow, Context};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use tokio::process::Command;

use crate::window::{WinType, Window};

use crate::config::General;
use crate::manager::WindowManager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session<WindowManager> {
    pub window_data: HashMap<String, Window>,
    config: General,
    manager: WindowManager,
}

impl<T: WindowManager> Session<T> {
    pub fn new(config: General, manager: T) -> Self {
        Self {
            window_data: HashMap::new(),
            config,
            manager,
        }
    }

    pub async fn save(&mut self, file: &str) -> anyhow::Result<()> {

        self.window_data = self.manager.update_session().await.0;
        self.window_data.iter_mut().for_each(|(_, w)| w.set_program_type(&self.config));

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file)
            .context(format!("Cannot open file {} for saving", file))?;
        let encoded: Vec<u8> = bincode::serialize(&self.window_data)
            .context(format!("Failed to serialize window data while saving"))?;

        let tasks: Vec<_> = self
            .window_data
            .iter()
            .filter_map(|(_, win)| match &win.wtype {
                WinType::CliApp(Some(cmd)) => {
                    Some(run_command(General::replace_cmd(&cmd.save_cmd, &win)))
                }
                WinType::App(app) => Some(run_command(General::replace_cmd(&app.save_cmd, &win))),
                _ => None,
            })
            .collect();

        for t in tasks {
            let _ = t.await;
        }

        file.write_all(&encoded)
            .context(format!("Failed to write session data to file {:?}", file))?;
        Ok(())
    }

    pub async fn load(&mut self, file: &str) -> anyhow::Result<()> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        self.window_data = bincode::deserialize(&buffer).context(format!(
            "Cannot deserialize bincode from saved session file {:?}",
            file
        ))?;

        let _tasks: Vec<_> = self
            .window_data
            .iter_mut()
            .filter_map(|(_, win)| match &win.wtype {
                WinType::CliApp(cmd) => {
                    win.program.cmdline = match cmd {
                        Some(cmd) => self.config.terminal.prepare_cli(&cmd.restore_cmd, win),
                        None => self.config.terminal.prepare_cli(&win.program.cmdline, win)
                    };
                    None
                }
                WinType::App(app) => Some(General::replace_cmd(&app.restore_cmd, win)),

                WinType::Terminal => None,
                _ => None,
            })
            .collect();
        self.manager.open_session(&self.window_data).await?;

        Ok(())
    }

    pub async fn replace(&mut self, file: &str) -> anyhow::Result<()> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        self.window_data = bincode::deserialize(&buffer).context(format!(
            "Cannot deserialize bincode from saved session file {:?}",
            file
        ))?;

        let _tasks: Vec<_> = self
            .window_data
            .iter_mut()
            .filter_map(|(_, win)| match &win.wtype {
                WinType::CliApp(cmd) => {
                    win.program.cmdline = match cmd {
                        Some(cmd) => self.config.terminal.prepare_cli(&cmd.restore_cmd, win),
                        None => self.config.terminal.prepare_cli(&win.program.cmdline, win)
                    };
                    None
                }
                WinType::App(app) => Some(General::replace_cmd(&app.restore_cmd, win)),

                WinType::Terminal => None,
                _ => None,
            })
            .collect();

        self.manager.close_session(&self.manager.update_session().await.0).await?;
        self.manager.open_session(&self.window_data).await?;
        Ok(())
    }
}

async fn run_command(cmdline: String) -> anyhow::Result<()> {
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
        Err(anyhow!(format!(
            "Command `{}` failed with exit code {:?}",
            cmdline,
            status.code()
        ))
        .into())
    }
}
