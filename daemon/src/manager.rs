use crate::window::Window;

pub enum SessionCmd {
    Open,
    Close,
}

pub trait WindowManager: Send + Sync {
    async fn send_command(&self, command: SessionCmd);
}
