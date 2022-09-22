use crate::AccessSharedData;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

#[derive(serde::Deserialize, serde::Serialize)]
struct TemperatureData {
    temperature_value: f32,
}
#[tracing::instrument(name = "getting the temperature", skip(common_data))]
pub async fn get_temperature(common_data: web::Data<AccessSharedData>) -> HttpResponse {
    let temperature = common_data.get_current_temp();
    tracing::info!("retreiving temperature of: {}", temperature);
    let temperature = TemperatureData {
        temperature_value: temperature,
    };
    let temperature = serde_json::to_string(&temperature).unwrap();
    tracing::debug!("json representation is: {}", temperature);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(temperature)
}
