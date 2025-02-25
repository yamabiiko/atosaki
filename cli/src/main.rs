use std::os::unix::net::UnixStream;
use std::io::prelude::*;

mod commands;

const SOCKET_PATH: &str = "/tmp/atosakid";

fn main() -> std::io::Result<()> {

    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    let send = vec![1];

    stream.write_all(&send)?;

    //todo!();
    Ok(())
}
