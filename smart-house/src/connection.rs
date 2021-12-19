use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type ConnectResult<T> = Result<T, ConnectError>;
