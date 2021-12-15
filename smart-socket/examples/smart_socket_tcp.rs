use std::io::{Read, Write};
use std::str::{self, FromStr};
use std::string::ToString;

use smart_socket::protocol::*;
use smart_socket::receiver::*;

enum Continuation {
    Continue,
    Stop,
}

fn handle_error(addr: &str, res: Result<(), std::io::Error>) -> Continuation {
    if let Err(e) = res {
        println!("got error from client {}: {}", addr, e);
        return Continuation::Stop;
    }
    Continuation::Continue
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = SmartSocketState::default();
    let receiver = SmartSocketReceiver::bind(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| DEFAULT_ADDRESS.to_string()),
    )?;

    for connection in receiver.incoming() {
        match connection {
            Ok(mut stream) => loop {
                let addr = stream.peer_addr().unwrap().to_string();
                // protocol expects commands with \r\n at the end:
                //  status\r\n
                //  switch\r\n
                let mut buf = vec![0; 8];
                if let Continuation::Stop = handle_error(&addr, stream.read_exact(&mut buf)) {
                    break;
                }

                let s = str::from_utf8(&buf)?;
                match ProtocolCommand::from_str(s) {
                    Ok(ProtocolCommand::Switch) => {
                        state.switch();
                        handle_error(&addr, stream.write_all(OK.as_bytes()));
                    }
                    Ok(ProtocolCommand::Status) => {
                        handle_error(&addr, write!(stream, "{}", state.status()));
                    }
                    Err(e) => {
                        handle_error(&addr, write!(stream, "{}", e));
                        break;
                    }
                };
            },
            Err(e) => return Err(Box::new(BindError::Io(e))),
        }
    }
    Ok(())
}
