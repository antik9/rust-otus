use std::str::{self, FromStr};

use thiserror::Error;

pub const OK: &str = "OK\r\n";

#[derive(Debug)]
pub enum ProtocolCommand {
    Switch,
    Status,
}

impl ToString for ProtocolCommand {
    fn to_string(&self) -> String {
        match self {
            ProtocolCommand::Switch => "switch\r\n".to_owned(),
            ProtocolCommand::Status => "status\r\n".to_owned(),
        }
    }
}

impl FromStr for ProtocolCommand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "switch" => Ok(ProtocolCommand::Switch),
            "status" => Ok(ProtocolCommand::Status),
            other => Err(ParseError::UnknownCommand(other.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unknown command: {0}\r\n")]
    UnknownCommand(String),
}
