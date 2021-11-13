use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct House {
    name: String,
    rooms: HashMap<String, Room>,
}

#[derive(Debug, Clone)]
pub struct Room {
    name: String,
    devices: HashMap<String, Device>,
}

#[derive(Debug, Clone)]
pub struct Device {
    name: String,
    description: String,
    device: DeviceType,
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Thermometer(Thermometer),
    SmartSocket(SmartSocket),
}

#[derive(Debug, Clone)]
pub struct Thermometer {
    temperature: f64,
}

#[derive(Debug, Clone)]
pub struct SmartSocket {
    is_on: bool,
    power: usize,
}

#[derive(Debug, Clone)]
pub struct Info {
    room: String,
    device: String,
    summary_info: String,
}

impl House {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: &str) -> &mut House {
        if self.rooms.get(name).is_some() {
            return self;
        }
        self.rooms.insert(name.to_owned(), Room::new(name));
        self
    }

    pub fn remove_room(&mut self, name: &str) -> &mut House {
        self.rooms.remove(name);
        self
    }

    pub fn get_rooms(&self) -> Vec<&Room> {
        let mut rooms: Vec<&Room> = Vec::new();
        for room in self.rooms.iter() {
            rooms.push(room.1);
        }
        rooms
    }

    pub fn get_room(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub fn get_info(self) -> Vec<Info> {
        let mut info: Vec<Info> = Vec::new();
        for room in self.rooms.iter() {
            for device in room.1.devices.iter() {
                info.push(Info {
                    room: room.1.name.to_owned(),
                    device: device.1.name.to_owned(),
                    summary_info: device.1.device.summary(),
                })
            }
        }
        info
    }
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: DeviceType, name: &str) -> &mut Room {
        if self.devices.get(name).is_some() {
            return self;
        }
        self.devices
            .insert(name.to_owned(), Device::new(name, device));
        self
    }

    pub fn remove_device(&mut self, name: &str) -> &mut Room {
        self.devices.remove(name);
        self
    }

    pub fn get_devices(&self) -> Vec<&Device> {
        let mut devices: Vec<&Device> = Vec::new();
        for device in self.devices.iter() {
            devices.push(device.1);
        }
        devices
    }

    pub fn get_socket(&mut self, name: &str) -> Option<&mut SmartSocket> {
        if let Some(d) = self.devices.get_mut(name) {
            match d.device {
                DeviceType::SmartSocket(ref mut s) => return Some(s),
                _ => return None,
            }
        }
        None
    }

    pub fn get_thermometer(&mut self, name: &str) -> Option<&mut Thermometer> {
        if let Some(d) = self.devices.get_mut(name) {
            match d.device {
                DeviceType::Thermometer(ref mut t) => return Some(t),
                _ => return None,
            }
        }
        None
    }
}

impl Device {
    pub fn new(name: &str, device: DeviceType) -> Self {
        Self {
            device,
            name: name.into(),
            description: "".into(),
        }
    }
}

impl DeviceType {
    pub fn summary(&self) -> String {
        match self {
            DeviceType::Thermometer(t) => format!("{}°C", t.get_temperature()),
            DeviceType::SmartSocket(s) => format!(
                "{} ({}W)",
                if s.is_on { "turned on" } else { "turned off" },
                s.get_consumed_power(),
            ),
        }
    }
}

impl Thermometer {
    pub fn new() -> Self {
        Self { temperature: 0.0 }
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
}

impl Default for Thermometer {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartSocket {
    pub fn new() -> Self {
        Self {
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

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
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
