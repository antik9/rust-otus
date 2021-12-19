use std::thread;
use std::time::Duration;

use thermometer::sender::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src_addr = std::env::args().nth(1).unwrap();
    let remote_addr = std::env::args().nth(2).unwrap();
    let name = std::env::args().nth(3).unwrap();
    let value: f64 = std::env::args().nth(4).unwrap().parse().unwrap();

    Sender::new(src_addr, remote_addr, name, value)?;
    loop {
        thread::sleep(Duration::new(1, 0));
    }
}
