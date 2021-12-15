use std::collections::HashMap;

use crate::devices::device::Device;
use crate::errors::HouseUpdateErr;
use crate::report::{HouseReport, Info};
use crate::room::Room;

#[derive(Debug)]
pub struct House {
    #[allow(dead_code)]
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

    pub fn get_rooms(&self) -> impl Iterator<Item = &Room> {
        self.rooms.iter().map(|kv| kv.1)
    }

    pub fn get_rooms_mut(&mut self) -> impl Iterator<Item = &mut Room> {
        self.rooms.iter_mut().map(|kv| kv.1)
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
                    room.get_name().to_string(),
                    device.get_name().to_string(),
                    device.summary(),
                ));
            }
        }
        HouseReport::new(report)
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::thread::sleep;
    use std::time::Duration;

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

    fn run_test<T>(test: T)
    where
        T: FnOnce(),
    {
        let mut cmd = Command::new("cargo")
            .args(vec![
                "run",
                "--manifest-path",
                "../smart-socket/Cargo.toml",
                "--example",
                "smart_socket_tcp",
                "--",
                "127.0.0.1:10702",
            ])
            .spawn()
            .unwrap();
        sleep(Duration::new(2, 0));

        test();
        cmd.kill().unwrap();
    }

    #[test]
    fn test_get_report() {
        run_test(|| {
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

            let socket_ = room.get_socket_mut(socket).unwrap();
            socket_.connect("127.0.0.1:10702").unwrap();
            socket_.switch().unwrap();

            let summary = house.get_report().summary();
            assert!(
                summary == "room: living room, device: socket near the bed, summary: turned on (2W)\nroom: living room, device: thermometer on the wall, summary: 0°C\n"
                || summary == "room: living room, device: thermometer on the wall, summary: 0°C\nroom: living room, device: socket near the bed, summary: turned on (2W)\n"
            );
        })
    }
}
