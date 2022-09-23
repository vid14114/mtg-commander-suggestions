use std::{time::Duration, thread};

use futures::{stream, StreamExt};
use thirtyfour::{
    components::{Component, ElementResolver, SelectElement},
    prelude::WebDriverResult,
    resolve, By, DesiredCapabilities, WebDriver, WebElement, resolve_present,
};

#[derive(Debug, Clone, Component)]
pub struct HomePageComponent {
    base: WebElement,
    #[by(css = "a.navigation:nth-child(4)")]
    tags_link: ElementResolver<WebElement>,
}

#[derive(Debug, Clone, Component)]
pub struct TagsPanel {
    base: WebElement,
    #[by(css = "select.tag-input-field")]
    tags_type_selector: ElementResolver<WebElement>,
    #[by(class = "tag-row-flex")]
    tags: ElementResolver<Vec<WebElement>>,
}

impl HomePageComponent {
    pub async fn open_tags_panel(&self) -> WebDriverResult<TagsPanel> {
        resolve!(self.tags_link).click().await?;
        Ok(TagsPanel::from(self.base_element()))
    }
}

impl TagsPanel {
    pub async fn get_tags(&self) -> WebDriverResult<Vec<String>> {
        let select_element = SelectElement::new(&resolve!(self.tags_type_selector)).await?;
        select_element.select_by_value("tag:ORACLE_CARD_TAG").await?;
        
        //Wait until the list refreshes with the selected card tags
        thread::sleep(Duration::from_millis(2000));

        let tags = resolve_present!(self.tags);
        let tag_names: Vec<String> = stream::unfold(tags.into_iter(), |mut tags| async {
            let tag_element = tags.next()?;
            if let Ok(a) = tag_element.find(By::Tag("a")).await {
                Some((a.text().await.ok()?, tags))
            } else {
                None
            }
        }).collect::<Vec<String>>().await;

        tag_names.iter().for_each(|tag| println!("{}", tag));
        Ok(tag_names)
    }
}

pub async fn fetch_tags() -> WebDriverResult<Vec<String>> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.goto("https://tagger.scryfall.com/").await?;
    let home_page = HomePageComponent::from(driver.find(By::Tag("body")).await?);
    let tags_panel = home_page.open_tags_panel().await?;
    let tags = tags_panel.get_tags().await?;
    driver.quit().await?;
    Ok(tags)
}
