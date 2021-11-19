use crate::devices::device::Device;

#[derive(Debug, Clone)]
pub struct SmartSocket {
    name: String,
    description: String,
    is_on: bool,
    power: usize,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_socket() {
        let mut socket = SmartSocket::new("socket", "description");
        assert_eq!(socket.is_on, false);
        socket.switch();
        assert_eq!(socket.is_on, true);
        socket.switch();
        assert_eq!(socket.is_on, false);
    }
}
