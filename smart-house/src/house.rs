use std::collections::HashMap;

use crate::errors::HouseUpdateErr;
use crate::report::HouseReport;
use crate::room::{Room, RoomsIter};

#[derive(Debug, Clone)]
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
        todo!()
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub fn get_report(&self) -> HouseReport {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_room() {
        let mut house = House::new("home");

        let name = "living room";
        house.add_room(name).unwrap();
        assert_eq!(house.get_room(name).is_some(), true);

        house.remove_room(name).unwrap();
        assert_eq!(house.get_room(name).is_none(), true);
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
}
