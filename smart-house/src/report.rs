#[derive(Debug)]
pub struct HouseReport {
    report: Vec<Info>,
}

#[derive(Debug)]
pub struct Info {
    room: String,
    device: String,
    summary_info: String,
}

impl HouseReport {
    pub fn new(report: Vec<Info>) -> Self {
        Self { report }
    }

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
    pub fn new(room: String, device: String, summary_info: String) -> Self {
        Self {
            room,
            device,
            summary_info,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "room: {}, device: {}, summary: {}",
            self.room, self.device, self.summary_info
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary() {
        let report = HouseReport::new(vec![
            Info {
                room: "bedroom".to_owned(),
                device: "socket".to_owned(),
                summary_info: "some info".to_owned(),
            },
            Info {
                room: "kitchen".to_owned(),
                device: "thermometer".to_owned(),
                summary_info: "some info".to_owned(),
            },
        ]);
        assert_eq!(
            report.summary(),
            "room: bedroom, device: socket, summary: some info\nroom: kitchen, device: thermometer, summary: some info\n",
        );
    }
}
