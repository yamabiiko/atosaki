use crate::window::Window;
use std::collections::HashMap;

pub enum SessionCmd {
    Save,
    Open,
    Close,
}

pub trait WindowManager: Send + Sync {
    async fn update_session(&self) -> (HashMap<String, Window>, anyhow::Result<bool>);
    async fn open_session(&self, w_data: &HashMap<String, Window>) -> anyhow::Result<bool>;
    async fn close_session(&self, w_data: &HashMap<String, Window>) -> anyhow::Result<bool>;
}
