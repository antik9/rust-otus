#[derive(Debug)]
pub enum ItemType {
    NewObject(),
    Int(String, i32),
    Str(String, String),
    EndObject(),
}

pub type ItemTypeVec = Vec<ItemType>;

pub trait ReportFormatter {
    fn format(&self, m: &ItemTypeVec) -> String;
}

pub struct PlainTextFormatter {}

impl ReportFormatter for PlainTextFormatter {
    fn format(&self, m: &ItemTypeVec) -> String {
        let mut result = "".to_owned();

        let mut add_comma = false;
        for (idx, item) in m.iter().enumerate() {
            match item {
                ItemType::NewObject() | ItemType::EndObject() => (),
                _ => {
                    if add_comma {
                        result.push_str(", ")
                    }
                }
            }

            match &item {
                ItemType::Int(key, i) => result.push_str(&format!("{}: {}", key, i)),
                ItemType::Str(key, s) => result.push_str(&format!("{}: {}", key, s)),
                ItemType::EndObject() => result.push('\n'),
                ItemType::NewObject() => (),
            }

            add_comma = idx != m.len() - 1;
            if let ItemType::NewObject() = item {
                add_comma = false;
            }
        }
        result
    }
}

pub struct JsonFormatter {}

impl ReportFormatter for JsonFormatter {
    fn format(&self, m: &ItemTypeVec) -> String {
        let mut result = "".to_owned();
        result.push('[');

        let mut add_comma = false;

        for (idx, item) in m.iter().enumerate() {
            if let ItemType::EndObject() = item {
            } else if add_comma {
                result.push_str(", ");
            }

            match &item {
                ItemType::Int(key, i) => result.push_str(&format!("\"{}\": {}", key, i)),
                ItemType::Str(key, s) => result.push_str(&format!("\"{}\": \"{}\"", key, s)),
                ItemType::NewObject() => result.push('{'),
                ItemType::EndObject() => result.push('}'),
            }

            add_comma = idx != m.len() - 1;
            if let ItemType::NewObject() = item {
                add_comma = false;
            }
        }
        result.push(']');
        result
    }
}
