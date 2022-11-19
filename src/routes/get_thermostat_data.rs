use crate::AccessSharedData;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

// sample JSON data: {"temperature":71.7116,"thermostat_setting":55,"thermostat_on":false}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ThermostatData {
    temperature: f32,
    thermostat_setting: u16,
    thermostat_on: bool,
}
#[tracing::instrument(name = "getting all the thermostat data", skip(common_data))]
pub async fn get_thermostat_data(common_data: web::Data<AccessSharedData>) -> HttpResponse {
    let thermostat_data = ThermostatData {
        temperature: common_data.current_temp(),
        thermostat_setting: common_data.thermostat_value(),
        thermostat_on: common_data.is_thermostat_on(),
    };
    tracing::info!("retreiving thermostat data of: {:?}", thermostat_data);

    let thermostat_data = serde_json::to_string(&thermostat_data).unwrap();
    tracing::debug!(
        "json representation of thermostat data is: {}",
        thermostat_data
    );
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(thermostat_data)
}
