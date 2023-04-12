use std::time::SystemTime;

use actix_web::{HttpResponse, post, Responder, web};
use bson::{doc as bson_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{Datelike, DateTime, NaiveDate, Utc};


use crate::Router;
use crate::Router::Intern::Database::MongoClient::Mongo;

#[derive(Deserialize)]
pub(crate) struct RequestWeather {
    plz: String,
    days: i32
}

#[derive(Clone, Serialize)]
pub(crate) struct WeatherData{
    date: String,
    maxTemp: f32,
    minTemp: f32,
    precipitation: f32,
    image: String
}

#[derive(Serialize)]
pub(crate) struct ResponseWeather {
    success: bool,
    message: String,
    data: Vec<WeatherData>
}

#[post("/weather")]
pub(crate) async fn weather(
    data: web::Json<RequestWeather>,
) -> Result<impl Responder, actix_web::Error> {

    if data.plz.is_empty() || data.days == 0 {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Missing data",
        })));
    }

    let plz = data.plz.clone();
    let days = data.days.clone();
    let mut weatherData: Vec<WeatherData> = Vec::new();
    let mut response = ResponseWeather{
        success: false,
        message: "".to_string(),
        data: weatherData.clone()
    };
    let client = reqwest::Client::new();
    let res = client.get("https://app-prod-ws.meteoswiss-app.ch/v1/plzDetail")
        .query(&[("plz", plz)])
        .send()
        .await
        .unwrap();
    let body = res.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    if let Some(forecast) = json["forecast"].as_array() {
        for day in forecast.iter().take(days as usize){
            let weather = WeatherData{
                //NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
                date: NaiveDate::parse_from_str(&*day["dayDate"].as_str().unwrap().to_string(), "%Y-%m-%d").unwrap().weekday().to_string(),
                maxTemp: day["temperatureMax"].as_f64().unwrap() as f32,
                minTemp: day["temperatureMin"].as_f64().unwrap() as f32,
                precipitation: day["precipitation"].as_f64().unwrap() as f32,
                image: "https://www.meteoswiss.admin.ch/static/product/resources/weather-symbols/".to_owned() + &day["iconDay"].as_i64().unwrap().to_string() + ".svg"
            };
            weatherData.push(weather);
        }
        response.success = true;
        response.message = "Weather data".to_string();
        response.data = weatherData;
    } else {
        response.success = false;
        response.message = "No weather data found".to_string();
    }

    Ok(HttpResponse::Ok().json(response))
}
