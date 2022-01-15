use crate::connection::ConnectResult;

#[async_trait::async_trait]
pub trait DeviceStatus: Device + Summary {}

#[async_trait::async_trait]
pub trait Summary {
    async fn summary(&self) -> String;
}

pub trait Device {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
}

#[async_trait::async_trait]
pub trait Switcher {
    async fn switch(&mut self) -> ConnectResult<()>;
}
