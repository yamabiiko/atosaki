use anyhow::{anyhow, Context};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use tokio::process::Command;

use crate::window::{WinType, WindowRegistry};

use crate::config::General;
use crate::manager::WindowManager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session<WindowManager> {
    registry: WindowRegistry,
    manager: WindowManager,
}

impl<T: WindowManager> Session<T> {
    pub fn new(config: General, manager: T) -> Self {
        Self {
            registry: WindowRegistry::new(config),
            manager,
        }
    }

    pub async fn save(&mut self, file: &str) -> anyhow::Result<()> {
        let fetched = self.manager.fetch_windows().await?;
        self.registry.update(fetched);
        // we only need to set this on save
        self.registry.set_program_type();
        self.registry.set_cmdline();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file)
            .context(format!("Cannot open file {} for saving", file))?;

        let encoded: Vec<u8> = bincode::serialize(&self.registry)
            .context(format!("Failed to serialize window data while saving"))?;

        for t in self.registry.on_save() {
            let _ = run_command(t).await;
        }

        file.write_all(&encoded)
            .context(format!("Failed to write session data to file {:?}", file))?;
        Ok(())
    }

    pub async fn load(&mut self, file: &str) -> anyhow::Result<()> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let saved_win: WindowRegistry = bincode::deserialize(&buffer).context(format!(
            "Cannot deserialize bincode from saved session file {:?}",
            file
        ))?;

        let fetched = self.manager.fetch_windows().await?;
        self.registry.update(fetched);

        let diff = saved_win.difference(&self.registry);

        self.manager.open_windows(diff.win_vec()).await?;
        Ok(())
    }

    pub async fn replace(&mut self, file: &str) -> anyhow::Result<()> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let saved_win: WindowRegistry = bincode::deserialize(&buffer).context(format!(
            "Cannot deserialize bincode from saved session file {:?}",
            file
        ))?;

        let fetched = self.manager.fetch_windows().await?;
        self.registry.update(fetched);

        self.manager.close_windows(self.registry.win_vec()).await?;
        self.manager.open_windows(saved_win.win_vec()).await?;
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
