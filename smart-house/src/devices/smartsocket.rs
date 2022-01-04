use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::str;
use std::sync::{Arc, Mutex};

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
    stream: Arc<Mutex<Option<TcpStream>>>,
}

#[derive(Debug, Default)]
pub struct SocketState {
    is_on: bool,
    power_consumption: usize,
}

impl SmartSocket {
    pub fn new(name: &str, description: &str) -> SmartSocket {
        Self {
            name: name.into(),
            description: description.into(),
            stream: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&mut self, addr: &str) -> ConnectResult<()> {
        *self.stream.lock().unwrap() = Some(TcpStream::connect(addr)?);
        Ok(())
    }

    async fn check_connection(&self) -> ConnectResult<()> {
        if self.stream.lock().unwrap().is_none() {
            return Err(ConnectError::Io(std::io::Error::new(
                ErrorKind::NotConnected,
                format!("no connection established to {}", self.name),
            )));
        }
        Ok(())
    }

    async fn get_status(&self) -> ConnectResult<SocketState> {
        self.check_connection().await?;
        self.stream
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .write_all(ProtocolCommand::Status.to_string().as_bytes())?;

        let mut buf = vec![0; 16];
        let n = self
            .stream
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .read(&mut buf)?;
        let s = str::from_utf8(&buf[..n]).unwrap();
        match Regex::new(r"is on \((\d+)W\)\r\n").unwrap().captures(s) {
            Some(group) => Ok(SocketState {
                is_on: true,
                power_consumption: group.get(1).unwrap().as_str().parse().unwrap(),
            }),
            None => Ok(SocketState::default()),
        }
    }

    pub async fn is_on(&self) -> ConnectResult<bool> {
        self.get_status().await.map(|res| res.is_on)
    }

    pub async fn switch(&mut self) -> ConnectResult<()> {
        self.check_connection().await?;
        self.stream
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .write_all(ProtocolCommand::Switch.to_string().as_bytes())?;
        let mut buf = vec![0; 4];
        self.stream
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .read_exact(&mut buf)?;
        Ok(())
    }

    pub async fn get_consumed_power(&self) -> ConnectResult<usize> {
        self.get_status().await.map(|res| res.power_consumption)
    }
}

#[async_trait::async_trait]
impl Device for SmartSocket {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
    async fn summary(&self) -> String {
        format!(
            "{} ({}W)",
            if self.is_on().await.unwrap() {
                "turned on"
            } else {
                "turned off"
            },
            self.get_consumed_power().await.unwrap(),
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
        sleep(Duration::new(10, 0));

        test();
        cmd.kill().unwrap();
    }

    #[test]
    fn test_switch_socket() {
        run_test(|| {
            let mut socket = SmartSocket::new("socket", "description");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                socket.connect("127.0.0.1:10703").await.unwrap();

                assert!(!socket.is_on().await.unwrap());
                socket.switch().await.unwrap();
                assert!(socket.is_on().await.unwrap());
                socket.switch().await.unwrap();
                assert!(!socket.is_on().await.unwrap());
            });
            rt.shutdown_background();
        })
    }
}
