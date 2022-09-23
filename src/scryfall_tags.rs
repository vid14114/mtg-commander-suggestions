use std::{thread, time::Duration};

use futures::{stream, StreamExt};
use thirtyfour::{
    components::{Component, ElementResolver, SelectElement},
    prelude::WebDriverResult,
    resolve, resolve_present, By, DesiredCapabilities, WebDriver, WebElement,
};

const TIMEOUT_MILLIS: u64 = 1000;

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
    #[by(css = "div.pagination:nth-child(3) > span:last-child > a:nth-child(1)")]
    last_page_anchor: ElementResolver<WebElement>,
    #[by(css = "div.pagination:nth-child(3)")]
    pagination_bar: ElementResolver<WebElement>,
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
        select_element
            .select_by_value("tag:ORACLE_CARD_TAG")
            .await?;

        //Wait until the list refreshes with the selected card tags
        thread::sleep(Duration::from_millis(TIMEOUT_MILLIS));

        let mut current_page = 1;
        let last_page: i32 = resolve!(self.last_page_anchor)
            .text()
            .await?
            .parse()
            .unwrap();
        let mut all_tags: Vec<String> = vec![];
        while current_page < last_page {
            let new_tags = self.fill_tags_from_current_page(&mut all_tags).await?;
            println!("PAGE {}", current_page);
            new_tags.iter().for_each(|tag| println!("{}", tag));
            resolve!(self.pagination_bar)
                .find(By::LinkText(&(current_page + 1).to_string()))
                .await?
                .click()
                .await?;
            //Wait until the list refreshes with the new page
            thread::sleep(Duration::from_millis(TIMEOUT_MILLIS));
            current_page += 1;
        }
        //fill in tags from last page
        self.fill_tags_from_current_page(&mut all_tags).await?;
        println!("Total tags: {}", all_tags.len());

        Ok(all_tags)
    }

    async fn fill_tags_from_current_page(
        &self,
        all_tags: &mut Vec<String>,
    ) -> WebDriverResult<Vec<String>> {
        let tags = resolve_present!(self.tags);
        let tag_names: Vec<String> = stream::unfold(tags.into_iter(), |mut tags| async {
            let tag_element = tags.next()?;
            if let Ok(a) = tag_element.find(By::Tag("a")).await {
                let text = a.text().await.ok()?;
                //ignore cycle tags
                if text.contains("cycle") {
                    None
                } else {
                    Some((text, tags))
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .await;
        all_tags.extend(tag_names.clone());
        Ok(tag_names)
    }
}

pub async fn fetch_tags() -> WebDriverResult<Vec<String>> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps)
        .await
        .expect("WebDriver connection");
    driver.goto("https://tagger.scryfall.com/").await?;
    let home_page = HomePageComponent::from(driver.find(By::Tag("body")).await?);
    let tags_panel = home_page.open_tags_panel().await?;
    let tags = tags_panel.get_tags().await?;
    driver.quit().await?;
    Ok(tags)
}
