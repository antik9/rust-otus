use std::collections::HashMap;

use crate::devices::device::Device;
use crate::devices::smartsocket::SmartSocket;
use crate::devices::thermometer::Thermometer;
use crate::devices::types::{DeviceType, DevicesIter};
use crate::result::{AddResult, RemoveResult};

pub struct RoomsIter {}

#[derive(Debug, Clone)]
pub struct Room {
    name: String,
    devices: HashMap<String, DeviceType>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_device(&mut self, device: DeviceType) -> AddResult {
        if self.devices.get(device.get_name()).is_none() {
            self.devices.insert(device.get_name().to_owned(), device);
        }
        AddResult {}
    }

    pub fn remove_device(&mut self, name: &str) -> RemoveResult {
        self.devices.remove(name);
        RemoveResult {}
    }

    pub fn get_devices(&self) -> DevicesIter {
        todo!()
    }

    pub fn get_socket(&self, name: &str) -> Option<&SmartSocket> {
        if let Some(DeviceType::SmartSocket(ref s)) = self.devices.get(name) {
            return Some(s);
        }
        None
    }

    pub fn get_socket_mut(&mut self, name: &str) -> Option<&mut SmartSocket> {
        if let Some(DeviceType::SmartSocket(ref mut s)) = self.devices.get_mut(name) {
            return Some(s);
        }
        None
    }

    pub fn get_thermometer(&self, name: &str) -> Option<&Thermometer> {
        if let Some(DeviceType::Thermometer(ref t)) = self.devices.get(name) {
            return Some(t);
        }
        None
    }

    pub fn get_thermometer_mut(&mut self, name: &str) -> Option<&mut Thermometer> {
        if let Some(DeviceType::Thermometer(ref mut t)) = self.devices.get_mut(name) {
            return Some(t);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_device() {
        let mut room = Room::new("bedroom");

        let name = "socket near the bed";
        room.add_device(DeviceType::SmartSocket(SmartSocket::new(name, "")));
        assert_eq!(room.get_socket(name).is_some(), true);

        room.remove_device(name);
        assert_eq!(room.get_socket(name).is_none(), true);
    }
}
