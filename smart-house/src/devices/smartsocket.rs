use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

use regex::Regex;
use smart_socket::protocol::ProtocolCommand;
use thiserror::Error;

use crate::devices::device::Device;

pub type ConnectResult<T> = Result<T, ConnectError>;

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug)]
pub struct SmartSocket {
    name: String,
    description: String,
    stream: TcpStream,
}

#[derive(Debug, Default)]
pub struct SocketState {
    is_on: bool,
    power_consumption: usize,
}

impl SmartSocket {
    pub fn new(name: &str, description: &str, addr: &str) -> ConnectResult<SmartSocket> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            name: name.into(),
            description: description.into(),
            stream,
        })
    }

    fn get_status(&mut self) -> ConnectResult<SocketState> {
        self.stream
            .write_all(ProtocolCommand::Status.to_string().as_bytes())?;

        let mut buf = vec![0; 16];
        let n = self.stream.read(&mut buf)?;
        let s = str::from_utf8(&buf[..n]).unwrap();
        match Regex::new(r"is on \((\d+)W\)\r\n").unwrap().captures(s) {
            Some(group) => Ok(SocketState {
                is_on: true,
                power_consumption: group.get(1).unwrap().as_str().parse().unwrap(),
            }),
            None => Ok(SocketState::default()),
        }
    }

    pub fn is_on(&mut self) -> ConnectResult<bool> {
        self.get_status().map(|res| res.is_on)
    }

    pub fn switch(&mut self) -> ConnectResult<()> {
        self.stream
            .write_all(ProtocolCommand::Switch.to_string().as_bytes())?;
        let mut buf = vec![0; 4];
        self.stream.read_exact(&mut buf)?;
        Ok(())
    }

    pub fn get_consumed_power(&mut self) -> ConnectResult<usize> {
        self.get_status().map(|res| res.power_consumption)
    }
}

impl Device for SmartSocket {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
    fn summary(&mut self) -> String {
        format!(
            "{} ({}W)",
            if self.is_on().unwrap() {
                "turned on"
            } else {
                "turned off"
            },
            self.get_consumed_power().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{process::Command, thread::sleep, time::Duration};

    use super::*;

    fn run_test<T>(test: T)
    where
        T: FnOnce(),
    {
        let mut cmd = Command::new("cargo")
            .args(vec![
                "run",
                "--manifest-path",
                "../smart-socket/Cargo.toml",
                "--example",
                "smart_socket_tcp",
                "--",
                "127.0.0.1:10703",
            ])
            .spawn()
            .unwrap();
        sleep(Duration::new(2, 0));

        test();
        cmd.kill().unwrap();
    }

    #[test]
    fn test_switch_socket() {
        run_test(|| {
            let mut socket = SmartSocket::new("socket", "description", "127.0.0.1:10703").unwrap();

            assert!(!socket.is_on().unwrap());
            socket.switch().unwrap();
            assert!(socket.is_on().unwrap());
            socket.switch().unwrap();
            assert!(!socket.is_on().unwrap());
        })
    }
}
