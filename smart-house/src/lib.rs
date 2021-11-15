use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct House {
    name: String,
    rooms: HashMap<String, Room>,
}

#[derive(Debug, Clone)]
pub struct Room {
    name: String,
    devices: HashMap<String, DeviceType>,
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Thermometer(Thermometer),
    SmartSocket(SmartSocket),
}

#[derive(Debug, Clone)]
pub struct Thermometer {
    name: String,
    description: String,
    temperature: f64,
}

#[derive(Debug, Clone)]
pub struct SmartSocket {
    name: String,
    description: String,
    is_on: bool,
    power: usize,
}

pub struct HouseReport {
    report: Vec<Info>,
}

pub struct Info {
    room: String,
    device: String,
    summary_info: String,
}

pub struct AddResult {}
pub struct RemoveResult {}
pub struct RoomsIter {}
pub struct DevicesIter {}

trait Device {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn summary(&self) -> String;
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
        let mut info: Vec<Info> = Vec::new();
        for room in self.rooms.iter() {
            for device in room.1.devices.iter() {
                info.push(Info {
                    room: room.1.name.to_owned(),
                    device: device.1.get_name().to_owned(),
                    summary_info: device.1.summary(),
                })
            }
        }
        HouseReport { report: info }
    }
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::new(),
        }
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

impl Thermometer {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            temperature: 0.0,
        }
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
}

impl Device for Thermometer {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
    fn summary(&self) -> String {
        format!("{}°C", self.get_temperature())
    }
}

impl SmartSocket {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            power: 0,
            is_on: false,
        }
    }

    pub fn switch(&mut self) {
        self.is_on = !self.is_on;
    }

    pub fn get_consumed_power(&self) -> usize {
        self.power
    }
}

impl Device for SmartSocket {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
    fn summary(&self) -> String {
        format!(
            "{} ({}W)",
            if self.is_on {
                "turned on"
            } else {
                "turned off"
            },
            self.get_consumed_power(),
        )
    }
}

impl HouseReport {
    pub fn summary(&self) -> String {
        let mut result = "".to_owned();
        for info in self.report.iter() {
            result += &info.summary().to_owned();
            result += "\n";
        }
        result
    }
}

impl Info {
    pub fn summary(&self) -> String {
        format!(
            "room: {}, device: {}, summary: {}",
            self.room, self.device, self.summary_info
        )
    }
}
