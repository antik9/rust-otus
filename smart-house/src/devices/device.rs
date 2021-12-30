#[async_trait::async_trait]
pub trait Device {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    async fn summary(&self) -> String;
}
