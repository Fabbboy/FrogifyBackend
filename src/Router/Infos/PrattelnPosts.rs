use std::cmp::min;

use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;


use crate::Router::FromApis::{scrape_pratteln_website, NewsItem}; // Import NewsItem here

#[derive(Deserialize)]
pub(crate) struct RequestNews {
    amount: i32
}

#[derive(Serialize)]
pub(crate) struct NewsData{
    success: bool,
    message: String,
    news: Vec<NewsItem> // Use NewsItem here
}

#[post("/news")]
pub(crate) async fn getNews(
    data: web::Json<RequestNews>,
) -> Result<impl Responder, actix_web::Error> {
    if data.amount == 0 {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        })));
    }

    let amount = data.amount.clone();

    let news_items = scrape_pratteln_website().await.unwrap(); // Use news_items directly

    let news_items_len = news_items.len(); // Get the length of news_items before calling into_iter
    let result: Vec<NewsItem> = news_items.into_iter().take(min(amount as usize, news_items_len)).collect();

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Successfully fetched news",
        "news": result
    })))
}
