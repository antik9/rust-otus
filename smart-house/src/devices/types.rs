use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;

use crate::devices::device::Device;
use crate::devices::smartsocket::SmartSocket;
use crate::devices::thermometer::Thermometer;

pub struct DevicesIter<'a> {
    base: Iter<'a, String, DeviceType>,
}

impl<'a> DevicesIter<'a> {
    pub fn new(devices: &'a HashMap<String, DeviceType>) -> Self {
        Self {
            base: devices.iter(),
        }
    }
}

impl<'a> Iterator for DevicesIter<'a> {
    type Item = &'a DeviceType;

    fn next(&mut self) -> Option<Self::Item> {
        match self.base.next() {
            Some(kv) => Some(kv.1),
            None => None,
        }
    }
}

pub struct DevicesIterMut<'a> {
    base: IterMut<'a, String, DeviceType>,
}

impl<'a> DevicesIterMut<'a> {
    pub fn new(devices: &'a mut HashMap<String, DeviceType>) -> Self {
        Self {
            base: devices.iter_mut(),
        }
    }
}

impl<'a> Iterator for DevicesIterMut<'a> {
    type Item = &'a mut DeviceType;

    fn next(&mut self) -> Option<Self::Item> {
        match self.base.next() {
            Some(kv) => Some(kv.1),
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
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
