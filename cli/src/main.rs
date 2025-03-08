use crate::commands::{Cli, Commands};
use clap::Parser;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

mod commands;

const SOCKET_PATH: &str = "/tmp/atosakid";

fn main() -> anyhow::Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;

    let args = Cli::parse();
    match args.command {
        Commands::Save => {
            stream.write_all(&vec![0])?;
        }
        Commands::Load => {
            stream.write_all(&vec![1])?;
        }
        Commands::Replace => {
            stream.write_all(&vec![2])?;
        }
    }

    Ok(())
}
