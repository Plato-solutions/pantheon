// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![cfg(feature = "thirtyfour_backend")]

use super::{Element, Locator, Webdriver};
use serde_json::Value as Json;
use std::time::Duration;
use thirtyfour::{
    components::select::SelectElement, prelude::ElementQueryable, By, OptionRect, ScriptArgs,
    WebDriverCommands,
};
use url::Url;

pub struct Client<'a>(pub &'a thirtyfour::WebDriver);

#[async_trait::async_trait]
impl<'a> Webdriver for Client<'a> {
    type Element = WebElement<'a>;
    type Error = crate::error::RunnerErrorKind;

    async fn goto(&mut self, url: &str) -> Result<(), Self::Error> {
        self.0.get(url).await?;
        Ok(())
    }

    async fn find(&mut self, locator: Locator) -> Result<Self::Element, Self::Error> {
        let e = self.0.find_element((&locator).into()).await?;
        Ok(WebElement(e, &self.0))
    }

    async fn find_all(&mut self, locator: Locator) -> Result<Vec<Self::Element>, Self::Error> {
        let elements = self
            .0
            .find_elements((&locator).into())
            .await?
            .into_iter()
            .map(move |e| WebElement(e, &self.0))
            .collect();
        Ok(elements)
    }

    async fn wait_for_visible(
        &mut self,
        locator: Locator,
        timeout: Duration,
    ) -> Result<(), Self::Error> {
        let locator: By = (&locator).into();
        let (e, _) = elapsed_fn(
            self.0
                .query(locator)
                .and_displayed()
                .wait(timeout, timeout / 3)
                .first(),
        )
        .await;
        e?;

        Ok(())
    }

    async fn wait_for_not_present(
        &mut self,
        locator: Locator,
        timeout: Duration,
    ) -> Result<(), Self::Error> {
        let locator: By = (&locator).into();
        let (e, _) = elapsed_fn(
            self.0
                .query(locator)
                .wait(timeout, timeout / 3)
                .not_exists(),
        )
        .await;
        e?;

        Ok(())
    }

    async fn wait_for_present(
        &mut self,
        locator: Locator,
        timeout: Duration,
    ) -> Result<(), Self::Error> {
        let locator: By = (&locator).into();
        let (e, _) = elapsed_fn(self.0.query(locator).wait(timeout, timeout / 3).exists()).await;
        e?;

        Ok(())
    }

    async fn wait_for_editable(
        &mut self,
        locator: Locator,
        timeout: Duration,
    ) -> Result<(), Self::Error> {
        let locator: By = (&locator).into();
        let (e, _) = elapsed_fn(
            self.0
                .query(locator)
                .wait(timeout, timeout / 3)
                .and_clickable()
                .and_enabled()
                .first(),
        )
        .await;
        e?;

        Ok(())
    }

    async fn current_url(&mut self) -> Result<Url, Self::Error> {
        let url = self.0.current_url().await?;
        Ok(Url::parse(&url).unwrap())
    }

    async fn set_window_size(&mut self, width: u32, height: u32) -> Result<(), Self::Error> {
        self.0
            .set_window_rect(OptionRect::new().with_size(width as i32, height as i32))
            .await?;
        Ok(())
    }

    async fn execute(&mut self, script: &str, a: Vec<Json>) -> Result<Json, Self::Error> {
        let mut args = ScriptArgs::new();
        for v in a {
            args.push_value(v);
        }

        let ret = self.0.execute_script_with_args(script, &args).await?;
        let json = ret.value().clone();

        Ok(json)
    }

    async fn close(&mut self) -> Result<(), Self::Error> {
        self.0.close().await?;
        Ok(())
    }
}

pub struct WebElement<'a>(thirtyfour::WebElement<'a>, &'a thirtyfour::WebDriver);

#[async_trait::async_trait]
impl<'a> Element for WebElement<'a> {
    type Driver = Client<'a>;
    type Error = crate::error::RunnerErrorKind;

    async fn attr(&mut self, attribute: &str) -> Result<Option<String>, Self::Error> {
        let attr = self.0.get_attribute(attribute).await?;
        Ok(attr)
    }

    async fn prop(&mut self, prop: &str) -> Result<Option<String>, Self::Error> {
        let prop = self.0.get_property(prop).await?;
        Ok(prop)
    }

    async fn text(&mut self) -> Result<String, Self::Error> {
        let text = self.0.text().await?;
        Ok(text)
    }

    async fn html(&mut self, inner: bool) -> Result<String, Self::Error> {
        let html = if inner {
            self.0.inner_html().await?
        } else {
            self.0.outer_html().await?
        };

        Ok(html)
    }

    async fn find(&mut self, search: Locator) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let search: By = (&search).into();
        let e = self.0.find_element(search).await?;

        Ok(Self(e, self.1))
    }

    async fn click(mut self) -> Result<Self::Driver, Self::Error> {
        self.0.click().await?;
        Ok(Client(self.1))
    }

    async fn select_by_index(mut self, index: usize) -> Result<Self::Driver, Self::Error> {
        SelectElement::new(&self.0)
            .await?
            .select_by_index(index as u32)
            .await?;

        Ok(Client(self.1))
    }

    async fn select_by_value(mut self, value: &str) -> Result<Self::Driver, Self::Error> {
        SelectElement::new(&self.0)
            .await?
            .select_by_value(value)
            .await?;

        Ok(Client(self.1))
    }
}

async fn elapsed_fn<F, R>(foo: F) -> (R, Duration)
where
    F: std::future::Future<Output = R>,
{
    let now = tokio::time::Instant::now();
    let result = foo.await;
    let elapsed = now.elapsed();

    (result, elapsed)
}

impl<'a> Into<By<'a>> for &'a Locator {
    fn into(self) -> By<'a> {
        match self {
            Locator::LinkText(s) => By::LinkText(&s),
            Locator::Css(s) => By::Css(&s),
            Locator::Id(s) => By::Id(&s),
            Locator::XPath(s) => By::XPath(&s),
        }
    }
}