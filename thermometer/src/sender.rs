use std::error::Error;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Sender {}

impl Sender {
    pub fn new(
        src_addr: String,
        remote_addr: String,
        name: String,
        value: f64,
    ) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(src_addr)?;

        thread::spawn(move || loop {
            let msg = format!("{}:\t{}", name, value);
            if let Err(e) = socket.send_to(&msg.to_string().into_bytes(), remote_addr.as_str()) {
                println!("cannot send data to {}: {}", remote_addr, e);
            }
            thread::sleep(Duration::from_millis(100));
        });
        Ok(Self {})
    }
}
