use thiserror::Error;

#[derive(Debug, Error)]
pub enum HouseUpdateErr {
    #[error("room \"{0}\" already exists")]
    RoomAlreadyExistsError(String),
    #[error("room \"{0}\" not found")]
    RoomNotFoundError(String),
    #[error("device \"{0}\" already exists")]
    DeviceAlreadyExistsError(String),
    #[error("device \"{0}\" not found")]
    DeviceNotFoundError(String),
}
