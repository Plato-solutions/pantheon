use std::time::Duration;

use crate::{
    error::RunnerErrorKind,
    webdriver::{Locator, Webdriver},
};

use super::Command;

pub struct WaitForElementEditable {
    target: Locator,
    timeout: Duration,
}

impl WaitForElementEditable {
    pub fn new(target: Locator, timeout: Duration) -> Self {
        Self { target, timeout }
    }
}

#[async_trait::async_trait]
impl<D: Webdriver> Command<D> for WaitForElementEditable {
    async fn run(&self, runner: &mut crate::runner::Runner<D>) -> Result<(), RunnerErrorKind> {
        runner
            .get_webdriver()
            .wait_for_editable(self.target.clone(), self.timeout)
            .await
            .map_err(|_| RunnerErrorKind::Timeout("WaitForElementPresent".to_owned()))?;

        Ok(())
    }
}
