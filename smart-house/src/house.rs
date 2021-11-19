use crate::report::HouseReport;
use crate::result::{AddResult, RemoveResult};
use crate::room::{Room, RoomsIter};
use std::collections::HashMap;

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

    pub fn add_room(&mut self, name: &str) -> AddResult {
        if self.rooms.get(name).is_none() {
            self.rooms.insert(name.to_owned(), Room::new(name));
        }
        AddResult {}
    }

    pub fn remove_room(&mut self, name: &str) -> RemoveResult {
        self.rooms.remove(name);
        RemoveResult {}
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
        house.add_room(name);
        assert_eq!(house.get_room(name).is_some(), true);

        house.remove_room(name);
        assert_eq!(house.get_room(name).is_none(), true);
    }
}
