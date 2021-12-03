use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;

use crate::devices::device::Device;
use crate::devices::smartsocket::SmartSocket;
use crate::devices::thermometer::Thermometer;
use crate::devices::types::{DeviceType, DevicesIter, DevicesIterMut};
use crate::errors::HouseUpdateErr;

pub struct RoomsIter<'a> {
    base: Iter<'a, String, Room>,
}

impl<'a> RoomsIter<'a> {
    pub fn new(rooms: &'a HashMap<String, Room>) -> Self {
        Self { base: rooms.iter() }
    }
}

impl<'a> Iterator for RoomsIter<'a> {
    type Item = &'a Room;

    fn next(&mut self) -> Option<Self::Item> {
        match self.base.next() {
            Some(kv) => Some(kv.1),
            None => None,
        }
    }
}

pub struct RoomsIterMut<'a> {
    base: IterMut<'a, String, Room>,
}

impl<'a> RoomsIterMut<'a> {
    pub fn new(rooms: &'a mut HashMap<String, Room>) -> Self {
        Self {
            base: rooms.iter_mut(),
        }
    }
}

impl<'a> Iterator for RoomsIterMut<'a> {
    type Item = &'a mut Room;

    fn next(&mut self) -> Option<Self::Item> {
        match self.base.next() {
            Some(kv) => Some(kv.1),
            None => None,
        }
    }
}

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

    pub fn add_device(&mut self, device: DeviceType) -> Result<(), HouseUpdateErr> {
        if self.devices.get(device.get_name()).is_none() {
            self.devices.insert(device.get_name().to_owned(), device);
            return Ok(());
        }
        Err(HouseUpdateErr::DeviceAlreadyExistsError(
            device.get_name().to_string(),
        ))
    }

    pub fn remove_device(&mut self, name: &str) -> Result<(), HouseUpdateErr> {
        if self.devices.get(name).is_some() {
            self.devices.remove(name);
            return Ok(());
        }
        Err(HouseUpdateErr::DeviceNotFoundError(name.to_string()))
    }

    pub fn get_devices(&self) -> DevicesIter {
        DevicesIter::new(&self.devices)
    }

    pub fn get_devices_mut(&mut self) -> DevicesIterMut {
        DevicesIterMut::new(&mut self.devices)
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
        room.add_device(DeviceType::SmartSocket(SmartSocket::new(name, "")))
            .unwrap();
        assert!(room.get_socket(name).is_some());

        room.remove_device(name).unwrap();
        assert!(room.get_socket(name).is_none());
    }

    #[test]
    fn test_error_on_remove_not_existing_device() {
        let mut room = Room::new("bedroom");
        let name = "socket near the bed";

        if let Err(HouseUpdateErr::DeviceNotFoundError(_)) = room.remove_device(name) {
            return;
        }
        panic!("remove not existing device from the room")
    }

    #[test]
    fn test_iterate_all_devices() {
        let mut room = Room::new("bedroom");
        let socket = "socket near the bed";
        let thermometer = "thermometer on the wall";

        room.add_device(DeviceType::SmartSocket(SmartSocket::new(socket, "")))
            .unwrap();
        room.add_device(DeviceType::Thermometer(Thermometer::new(thermometer, "")))
            .unwrap();

        let mut has_socket = false;
        let mut has_thermometer = false;
        for device in room.get_devices() {
            match device.get_name() {
                "socket near the bed" => has_socket = true,
                "thermometer on the wall" => has_thermometer = true,
                _ => panic!("unexpected device in the room"),
            }
        }

        assert!(has_socket);
        assert!(has_thermometer);
    }
}
