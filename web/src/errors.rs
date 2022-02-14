#[derive(serde::Serialize)]
pub struct JsonError {
    error: String,
}

impl JsonError {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}
