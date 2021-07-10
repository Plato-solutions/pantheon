// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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

pub struct WaitForElementNotEditable {
    target: Locator,
    timeout: Duration,
}

impl WaitForElementNotEditable {
    pub fn new(target: Locator, timeout: Duration) -> Self {
        Self { target, timeout }
    }
}

#[async_trait::async_trait]
impl<D: Webdriver> Command<D> for WaitForElementNotEditable {
    async fn run(&self, runner: &mut crate::runner::Runner<D>) -> Result<(), RunnerErrorKind> {
        runner
            .get_webdriver()
            .wait_for_not_editable(self.target.clone(), self.timeout)
            .await
            .map_err(|_| RunnerErrorKind::Timeout("WaitForElementPresent".to_owned()))?;

        Ok(())
    }
}
