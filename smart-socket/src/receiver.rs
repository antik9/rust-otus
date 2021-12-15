use std::io;
use std::net::{Incoming, TcpListener, ToSocketAddrs};

use thiserror::Error;

pub const DEFAULT_ADDRESS: &str = "127.0.0.1:10701";

#[derive(Default)]
pub struct SmartSocketState {
    is_on: bool,
}

impl SmartSocketState {
    pub fn status(&self) -> String {
        match self.is_on {
            true => "is on (2W)\r\n".to_owned(),
            false => "is off\r\n".to_owned(),
        }
    }

    pub fn switch(&mut self) {
        self.is_on = !self.is_on;
    }
}

pub type BindResult = Result<SmartSocketReceiver, BindError>;

pub struct SmartSocketReceiver {
    tcp: TcpListener,
}

impl SmartSocketReceiver {
    pub fn bind<Addrs>(addr: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addr)?;
        Ok(Self { tcp })
    }

    pub fn incoming(&self) -> Incoming<'_> {
        self.tcp.incoming()
    }
}

#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
