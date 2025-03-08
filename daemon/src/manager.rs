use crate::window::Window;
use std::collections::BTreeSet;

pub trait WindowManager: Send + Sync {
    async fn fetch_windows(&self) -> anyhow::Result<Vec<Window>>;
    async fn open_windows(&self, wins: Vec<&Window>) -> anyhow::Result<bool>;
    async fn close_windows(&self, wins: Vec<&Window>) -> anyhow::Result<bool>;
    async fn toggle_float(&self, wins: Vec<&Window>) -> anyhow::Result<bool>;
}
