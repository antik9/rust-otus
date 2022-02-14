use crate::formatter::{ItemTypeVec, ReportFormatter};

pub struct HouseReport {
    report: ItemTypeVec,
    fmt: Box<dyn ReportFormatter>,
}

impl HouseReport {
    pub fn new(report: ItemTypeVec, fmt: Box<dyn ReportFormatter>) -> Self {
        Self { report, fmt }
    }

    pub fn summary(&self) -> String {
        self.fmt.format(&self.report)
    }
}

#[cfg(test)]
mod tests {
    use crate::formatter::{ItemType, JsonFormatter, PlainTextFormatter};

    use super::*;

    #[test]
    fn test_summary_plain_text() {
        let report = HouseReport::new(
            vec![
                ItemType::NewObject(),
                ItemType::Str("room".into(), "bedroom".into()),
                ItemType::Str("device".into(), "socket".into()),
                ItemType::Str("summary".into(), "some info".into()),
                ItemType::EndObject(),
                ItemType::NewObject(),
                ItemType::Str("room".into(), "kitchen".into()),
                ItemType::Str("device".into(), "thermometer".into()),
                ItemType::Str("summary".into(), "some info".into()),
                ItemType::EndObject(),
            ],
            Box::new(PlainTextFormatter {}),
        );
        assert_eq!(
            report.summary(),
            "room: bedroom, device: socket, summary: some info\nroom: kitchen, device: thermometer, summary: some info\n",
        );
    }

    #[test]
    fn test_summary_json() {
        let report = HouseReport::new(
            vec![
                ItemType::NewObject(),
                ItemType::Str("room".into(), "bedroom".into()),
                ItemType::Str("device".into(), "socket".into()),
                ItemType::Str("summary".into(), "some info".into()),
                ItemType::EndObject(),
                ItemType::NewObject(),
                ItemType::Str("room".into(), "kitchen".into()),
                ItemType::Str("device".into(), "thermometer".into()),
                ItemType::Str("summary".into(), "some info".into()),
                ItemType::EndObject(),
            ],
            Box::new(JsonFormatter {}),
        );
        assert_eq!(
            report.summary(),
            "[{\"room\": \"bedroom\", \"device\": \"socket\", \"summary\": \"some info\"}, {\"room\": \"kitchen\", \"device\": \"thermometer\", \"summary\": \"some info\"}]",
        );
    }
}
