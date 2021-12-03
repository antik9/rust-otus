use std::collections::HashMap;

use crate::devices::device::Device;
use crate::errors::HouseUpdateErr;
use crate::report::{HouseReport, Info};
use crate::room::{Room, RoomsIter, RoomsIterMut};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct House {
    name: String,
    rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: &str) -> Result<(), HouseUpdateErr> {
        if self.rooms.get(name).is_none() {
            self.rooms.insert(name.to_owned(), Room::new(name));
            return Ok(());
        }
        Err(HouseUpdateErr::RoomAlreadyExistsError(name.to_string()))
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), HouseUpdateErr> {
        if self.rooms.get(name).is_some() {
            self.rooms.remove(name);
            return Ok(());
        }
        Err(HouseUpdateErr::RoomNotFoundError(name.to_string()))
    }

    pub fn get_rooms(&self) -> RoomsIter {
        RoomsIter::new(&self.rooms)
    }

    pub fn get_rooms_mut(&mut self) -> RoomsIterMut {
        RoomsIterMut::new(&mut self.rooms)
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub fn get_report(&self) -> HouseReport {
        let mut report: Vec<Info> = Vec::new();
        for room in self.get_rooms() {
            for device in room.get_devices() {
                report.push(Info::new(
                    room.get_name(),
                    device.get_name(),
                    device.summary(),
                ));
            }
        }
        HouseReport::new(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::smartsocket::SmartSocket;
    use crate::devices::thermometer::Thermometer;
    use crate::devices::types::DeviceType;

    #[test]
    fn test_add_remove_room() {
        let mut house = House::new("home");

        let name = "living room";
        house.add_room(name).unwrap();
        assert!(house.get_room(name).is_some());

        house.remove_room(name).unwrap();
        assert!(house.get_room(name).is_none());
    }

    #[test]
    fn test_error_on_adding_existing_room() {
        let mut house = House::new("home");
        let name = "living room";

        house.add_room(name).unwrap();
        if let Err(HouseUpdateErr::RoomAlreadyExistsError(_)) = house.add_room(name) {
            return;
        }
        panic!("adding already existing room to the house")
    }

    #[test]
    fn test_iterate_all_rooms() {
        let mut house = House::new("home");
        let living_room = "living room";
        let kitchen = "kitchen";

        house.add_room(living_room).unwrap();
        house.add_room(kitchen).unwrap();

        let mut has_living_room = false;
        let mut has_kitchen = false;
        for room in house.get_rooms() {
            match room.get_name() {
                "living room" => has_living_room = true,
                "kitchen" => has_kitchen = true,
                _ => panic!("unexpected room in the house"),
            }
        }

        assert!(has_living_room);
        assert!(has_kitchen);
    }

    #[test]
    fn test_change_room_in_iter_mut() {
        let mut house = House::new("home");
        let living_room = "living room";
        let socket = "socket near the bed";

        house.add_room(living_room).unwrap();

        for room in house.get_rooms_mut() {
            room.add_device(DeviceType::SmartSocket(SmartSocket::new(socket, "")))
                .unwrap();
        }

        let room = house.get_room(living_room).unwrap();
        let device = room.get_socket(socket).unwrap();
        assert_eq!(device.get_name(), socket);
    }

    #[test]
    fn test_get_report() {
        let mut house = House::new("home");
        let living_room = "living room";
        let socket = "socket near the bed";
        let thermometer = "thermometer on the wall";

        house.add_room(living_room).unwrap();
        let room = house.get_room_mut(living_room).unwrap();

        room.add_device(DeviceType::SmartSocket(SmartSocket::new(socket, "")))
            .unwrap();
        room.add_device(DeviceType::Thermometer(Thermometer::new(thermometer, "")))
            .unwrap();

        room.get_socket_mut(socket).unwrap().switch();

        let summary = house.get_report().summary();
        assert!(
            summary == "room: living room, device: socket near the bed, summary: turned on (0W)\nroom: living room, device: thermometer on the wall, summary: 0°C\n" 
            || summary == "room: living room, device: thermometer on the wall, summary: 0°C\nroom: living room, device: socket near the bed, summary: turned on (0W)\n"
        );
    }
}
