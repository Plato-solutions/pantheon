use super::Command;
use crate::{error::RunnerErrorKind, webdriver::Webdriver};

pub struct AssertAlert {
    text: String,
}

impl AssertAlert {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

#[async_trait::async_trait]
impl Command for AssertAlert {
    async fn run<D, E>(&self, runner: &mut crate::runner::Runner<D>) -> Result<(), RunnerErrorKind>
    where
        D: Webdriver<Element = E> + Send,
        E: crate::webdriver::Element<Driver = D> + Send,
    {
        let alert = runner.get_webdriver().alert_text().await?;

        if alert == self.text {
            Ok(())
        } else {
            Err(RunnerErrorKind::AssertFailed {
                lhs: alert,
                rhs: self.text.clone(),
            })
        }
    }
}
