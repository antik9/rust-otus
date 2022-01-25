use crate::connection::ConnectResult;

use super::device::Switcher;

pub struct Retry {
    attempts: usize,
    inner: Box<dyn Switcher + Send>,
}

impl Retry {
    pub fn new(inner: Box<dyn Switcher + Send>, attempts: usize) -> Self {
        Self { inner, attempts }
    }
}

#[async_trait::async_trait]
impl Switcher for Retry {
    async fn switch(&mut self) -> ConnectResult<()> {
        let mut res: ConnectResult<()> = Ok(());
        for i in 0..self.attempts {
            res = self.inner.switch().await;
            if let Ok(()) = res {
                println!("succeed on the {} attempt", i + 1);
                break;
            }
        }
        res
    }
}
