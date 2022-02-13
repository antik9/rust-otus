use std::collections::HashMap;
use std::net::UdpSocket;
use std::str;
use std::sync::{mpsc, Arc, RwLock};
use std::time::Duration;

use regex::Regex;

use crate::connection::ConnectResult;

#[derive(Debug)]
pub struct Receiver {
    data: Arc<RwLock<HashMap<String, f64>>>,
    done: mpsc::Sender<bool>,
}

impl Receiver {
    #[allow(unreachable_code)]
    pub async fn new(addr: &str) -> ConnectResult<Receiver> {
        let data = Arc::new(RwLock::new(HashMap::new()));
        let _data = data.clone();
        let (done, cancel) = mpsc::channel();

        let socket = UdpSocket::bind(addr).unwrap();
        #[cfg(feature = "no-tokio")]
        {
            use std::thread;
            thread::spawn(move || {
                let mut buf: Vec<u8> = vec![0; 64];
                loop {
                    let (n, _) = socket.recv_from(&mut buf).unwrap();
                    let s = str::from_utf8(&buf[..n]).unwrap();

                    if let Some(group) = Regex::new(r"([\w\s_-]+):\t(\d+)").unwrap().captures(s) {
                        _data.write().unwrap().insert(
                            group.get(1).unwrap().as_str().to_owned(),
                            group.get(2).unwrap().as_str().parse().unwrap(),
                        );
                    }
                    if cancel.recv_timeout(Duration::from_millis(1)).is_ok() {
                        println!("closing receiver...");
                        break;
                    }
                }
            });
            return Ok(Self { data, done });
        }

        tokio::spawn(async move {
            let mut buf: Vec<u8> = vec![0; 64];
            loop {
                let (n, _) = socket.recv_from(&mut buf).unwrap();
                let s = str::from_utf8(&buf[..n]).unwrap();

                if let Some(group) = Regex::new(r"([\w\s_-]+):\t(\d+)").unwrap().captures(s) {
                    _data.write().unwrap().insert(
                        group.get(1).unwrap().as_str().to_owned(),
                        group.get(2).unwrap().as_str().parse().unwrap(),
                    );
                }
                if cancel.recv_timeout(Duration::from_millis(1)).is_ok() {
                    println!("closing receiver...");
                    break;
                }
            }
        });
        Ok(Self { data, done })
    }

    pub fn get_data(&self, name: &str) -> Option<f64> {
        self.data.read().unwrap().get(name).map(|v| v.to_owned())
    }
}

impl Drop for Receiver {
    fn drop(&mut self) {
        self.done.send(true).unwrap();
    }
}
