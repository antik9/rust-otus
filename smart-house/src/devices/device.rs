pub trait Device {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn summary(&mut self) -> String;
}
