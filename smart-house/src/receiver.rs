use std::collections::HashMap;
use std::net::UdpSocket;
use std::str;
use std::sync::{Arc, RwLock};
use std::thread;

use regex::Regex;

use crate::connection::ConnectResult;

#[derive(Debug)]
pub struct Datagram {
    pub serial: String,
    pub temperature: f64,
}

#[derive(Debug)]
pub struct Receiver {
    data: Arc<RwLock<HashMap<String, f64>>>,
}

impl Receiver {
    pub fn new(addr: &str) -> ConnectResult<Receiver> {
        let data = Arc::new(RwLock::new(HashMap::new()));
        let _data = data.clone();

        let socket = UdpSocket::bind(addr)?;
        thread::spawn(move || {
            let mut buf: Vec<u8> = Vec::with_capacity(100);
            loop {
                let (n, _) = socket.recv_from(&mut buf).unwrap();
                let s = str::from_utf8(&buf[..n]).unwrap();

                if let Some(group) = Regex::new(r"([\w\s]+):\t\d+").unwrap().captures(s) {
                    _data.write().unwrap().insert(
                        group.get(1).unwrap().as_str().to_owned(),
                        group.get(2).unwrap().as_str().parse().unwrap(),
                    );
                }
            }
        });

        Ok(Self { data })
    }

    pub fn get_data(&self, name: &str) -> Option<f64> {
        self.data.read().unwrap().get(name).map(|v| v.to_owned())
    }
}
