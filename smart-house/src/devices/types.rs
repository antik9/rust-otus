use crate::devices::device::Device;
use crate::devices::smartsocket::SmartSocket;
use crate::devices::thermometer::Thermometer;

pub struct DevicesIter {}

#[derive(Debug)]
pub enum DeviceType {
    Thermometer(Thermometer),
    SmartSocket(SmartSocket),
}

impl Device for DeviceType {
    fn get_name(&self) -> &str {
        match self {
            DeviceType::Thermometer(t) => t.get_name(),
            DeviceType::SmartSocket(s) => s.get_name(),
        }
    }

    fn get_description(&self) -> &str {
        match self {
            DeviceType::Thermometer(t) => t.get_description(),
            DeviceType::SmartSocket(s) => s.get_description(),
        }
    }

    fn summary(&self) -> String {
        match self {
            DeviceType::Thermometer(t) => t.summary(),
            DeviceType::SmartSocket(s) => s.summary(),
        }
    }
}
