use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct NewsItem {
    pub(crate) date: String,
    pub(crate) title: String,
    pub(crate) link: String,
    pub(crate) description: String,
}

pub(crate) async fn scrape_pratteln_website() -> Result<Vec<NewsItem>, Box<dyn Error>> {
    let url = "https://www.pratteln.ch/";

    let response = reqwest::get(url).await?;
    let mut news_items = Vec::new();

    if response.status().is_success() {
        let body = response.text().await?;
        let document = Html::parse_document(&body);

        let news_slider_selector = Selector::parse(".news-slider").unwrap();
        let slider_item_container_selector = Selector::parse(".slider-item-container").unwrap();
        let date_selector = Selector::parse(".icms-date").unwrap();
        let title_selector = Selector::parse("h2").unwrap();
        let link_selector = Selector::parse("h2 a").unwrap();

        let description_selector = Selector::parse(".icms-beschreibung").unwrap();

        if let Some(news_slider) = document.select(&news_slider_selector).next() {
            for item in news_slider.select(&slider_item_container_selector) {
                let date = item
                    .select(&date_selector)
                    .next()
                    .map(|element| element.text().collect::<String>().trim().to_owned())
                    .unwrap_or_else(|| "No date available".to_string());
                let title = item
                    .select(&title_selector)
                    .next()
                    .map(|element| element.text().collect::<String>().trim().to_owned());
                let link = item
                    .select(&link_selector)
                    .next()
                    .and_then(|element| element.value().attr("href").map(|link| format!("https://www.pratteln.ch{}", link)));
                let description = item
                    .select(&description_selector)
                    .next()
                    .map(|element| element.text().collect::<String>().trim().to_owned())
                    .unwrap_or_else(|| "No description available".to_string());

                if let (Some(title), Some(link)) = (title, link) {
                    let news_item = NewsItem {
                        date,
                        title,
                        link,
                        description,
                    };
                    news_items.push(news_item);
                }
            }
        }
    }
    Ok(news_items)
}

