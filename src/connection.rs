use std::{
    io::{self, Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

use thiserror::Error;
// pub enum RunHyprError {}

pub struct Connection {
    write: UnixStream,
}
impl Connection {
    pub fn new() -> Connection {
        let his = std::env!("HYPRLAND_INSTANCE_SIGNATURE");
        Connection {
            write: UnixStream::connect(PathBuf::from(format!("/tmp/hypr/{his}/.socket.sock")))
                .expect("could not connect"),
        }
    }
    pub fn send_with_warning(&mut self, command: String) {
        if let Err(e) = send_raw(&mut self.write, command.to_owned()) {
            eprintln!("::ERROR::> {command} {e}")
        };
    }

    pub fn send_with_panic(&mut self, command: String) {
        if let Err(e) = send_raw(&mut self.write, command) {
            panic!("::ERROR::> {e}")
        };
    }
}

fn send_raw(socket: &mut UnixStream, command: String) -> Result<(), SendError> {
    // let buf = [0;1024]
    match socket.write(command.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(SendError::WriteFailed(e)),
    };

    let mut resp = String::new();
    match socket.read_to_string(&mut resp) {
        Ok(_) => {
            println!("::RESP::\t{command}\t{}", resp);
            return Ok(());
        }
        Err(e) => return Err(SendError::ReadFailed(e)),
    };
}

#[derive(Debug, Error)]
enum SendError {
    #[error("failed to read: {0}")]
    ReadFailed(#[from] io::Error),
    #[error("failed to write: {0}")]
    WriteFailed(io::Error),
}

#[derive(Debug, Error)]
enum ReadResponseError {
    #[error("failed to read the reponse from write socket: {0}")]
    FailedToRead(#[from] io::Error),
}
