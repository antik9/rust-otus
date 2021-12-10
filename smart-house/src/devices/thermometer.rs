use crate::devices::device::Device;

#[derive(Debug)]
pub struct Thermometer {
    name: String,
    description: String,
    temperature: f64,
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
    fn summary(&mut self) -> String {
        format!("{}°C", self.get_temperature())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let thermometer = Thermometer::new("t", "description");
        assert_eq!(thermometer.get_temperature(), 0.0);
    }
}
