use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct General {
    apps: Vec<App>,
    cmds: Vec<Cmd>,
    terminal: Terminal,
}

#[derive(Debug, Deserialize)]
struct Cmd {
    exe: String,
    save_cmd: String,
    restore_cmd: String
}

#[derive(Debug, Deserialize)]
struct App {
    class: String,
    title: String,
    save_cmd: String,
    restore_cmd: String
}

#[derive(Debug, Deserialize)]
struct Terminal {
    class: String,
}
