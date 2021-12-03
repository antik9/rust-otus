use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum HouseUpdateErr {
    RoomAlreadyExistsError(String),
    RoomNotFoundError(String),
    DeviceAlreadyExistsError(String),
    DeviceNotFoundError(String),
}

impl Display for HouseUpdateErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HouseUpdateErr::RoomAlreadyExistsError(s) => write!(f, "room \"{}\" already exists", s),
            HouseUpdateErr::RoomNotFoundError(s) => write!(f, "room \"{}\" not found", s),
            HouseUpdateErr::DeviceAlreadyExistsError(s) => {
                write!(f, "device \"{}\" already exists", s)
            }
            HouseUpdateErr::DeviceNotFoundError(s) => write!(f, "device \"{}\" not found", s),
        }
    }
}

impl Error for HouseUpdateErr {}
