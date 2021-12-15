use std::rc::Rc;

use crate::{devices::device::Device, receiver::Receiver};

#[derive(Debug)]
pub struct Thermometer {
    name: String,
    description: String,
    receiver: Rc<Option<Receiver>>,
}

impl Thermometer {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            receiver: Rc::new(None),
        }
    }

    pub fn add_receiver(&mut self, receiver: Rc<Option<Receiver>>) {
        self.receiver = receiver;
    }

    pub fn get_temperature(&self) -> f64 {
        if let Some(receiver) = &*self.receiver {
            return receiver.get_data(&self.name).unwrap_or(0.0);
        }
        0.0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let thermometer = Thermometer::new("t", "description");
        assert!(thermometer.get_temperature() < f64::EPSILON);
    }
}
