use crate::shared_data::SharedData;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use std::sync::{Arc, Mutex};

#[derive(serde::Deserialize, serde::Serialize)]
struct TemperatureData {
    temperature_value: f32,
}

pub async fn get_temperature(common_data: web::Data<Arc<Mutex<SharedData>>>) -> HttpResponse {
    let temperature = common_data.lock().unwrap().current_temp;
    let temperature = TemperatureData {
        temperature_value: temperature,
    };
    let temperature = serde_json::to_string(&temperature).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(temperature)
}
