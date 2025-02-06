//use std::process; // replace with nix ?

struct WindowData {
    window_id: u64,
    at: [u16; 2],
    size: [u16; 2],
    monitor: u16,
    workspace: u16,
    address: String,
    class: String,
    title: String,
    init_class: String,
    init_title: String,
    shell_id: u32,
    fullscreen: bool,
    program: Program,
}

struct Program {
    shell_id: u32,
    cwd: String,
    exe: String,
    cmdline: String,
    pid: u32,
}
