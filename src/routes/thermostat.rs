use crate::AccessSharedData;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ThermostatData {
    thermostat_setting: usize,
}
#[tracing::instrument(
    name = "getting the thermostat",
    skip(form, shared_data),
    fields(
        new_therm_data = %form.thermostat_setting
    )
)]
pub async fn set_thermostat(
    form: web::Form<ThermostatData>,
    shared_data: web::Data<AccessSharedData>,
) -> HttpResponse {
    shared_data.set_thermostat_value(form.thermostat_setting);
    tracing::info!("New thermostat value: {}", form.thermostat_setting);
    HttpResponse::Ok().finish()
}
#[tracing::instrument(name = "setting the thermostat", skip(shared_data))]
pub async fn get_thermostat(shared_data: web::Data<AccessSharedData>) -> HttpResponse {
    let thermostat = shared_data.get_thermostat_value();
    tracing::debug!("Thermostat value: {}", thermostat);
    let thermostat = ThermostatData {
        thermostat_setting: thermostat,
    };
    let thermostat = serde_json::to_string(&thermostat).unwrap();
    tracing::debug!("Thermostat json: {}", thermostat);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(thermostat)
}
