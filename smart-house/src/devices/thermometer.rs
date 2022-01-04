use std::sync::{Arc, Mutex};

use crate::{devices::device::Device, receiver::Receiver};

#[derive(Debug)]
pub struct Thermometer {
    name: String,
    description: String,
    receiver: Arc<Mutex<Option<Receiver>>>,
}

impl Thermometer {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            receiver: Arc::new(Mutex::new(None)),
        }
    }

    pub fn add_receiver(&mut self, receiver: Arc<Mutex<Option<Receiver>>>) {
        self.receiver = receiver;
    }

    pub fn get_temperature(&self) -> f64 {
        if let Some(receiver) = &*self.receiver.lock().unwrap() {
            return receiver.get_data(&self.name).unwrap_or(0.0);
        }
        0.0
    }
}

#[async_trait::async_trait]
impl Device for Thermometer {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
    async fn summary(&self) -> String {
        format!("{}°C", self.get_temperature())
    }
}

#[cfg(test)]
mod tests {
    use std::{process::Command, thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn test_get_temperature() {
        let thermometer = Thermometer::new("t", "description");
        assert!(thermometer.get_temperature() < f64::EPSILON);
    }

    fn run_test<T>(test: T)
    where
        T: FnOnce(),
    {
        let mut cmd = Command::new("cargo")
            .args(vec![
                "run",
                "--manifest-path",
                "../thermometer/Cargo.toml",
                "--example",
                "thermometer_udp",
                "--",
                "127.0.0.1:11601",
                "127.0.0.1:11701",
                "thermometer on the wall",
                "25",
            ])
            .spawn()
            .unwrap();
        sleep(Duration::new(5, 0));

        test();
        cmd.kill().unwrap();
    }

    #[test]
    fn test_get_temperature_from_receiver() {
        run_test(|| {
            let mut thermometer = Thermometer::new("thermometer on the wall", "");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let receiver = Receiver::new("127.0.0.1:11701").await.unwrap();
                thermometer.add_receiver(Arc::new(Mutex::new(Some(receiver))));

                sleep(Duration::from_millis(200));
                assert!((thermometer.get_temperature() - 25.0).abs() < f64::EPSILON);
            });
            rt.shutdown_background();
        })
    }
}
