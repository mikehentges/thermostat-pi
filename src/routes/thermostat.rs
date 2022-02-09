use crate::AccessSharedData;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;


#[derive(serde::Deserialize, serde::Serialize)]
pub struct ThermostatData {
    thermostat_setting: usize,
}

pub async fn set_thermostat(
    form: web::Form<ThermostatData>,
    shared_data: web::Data<AccessSharedData>,
) -> HttpResponse {
    shared_data.set_thermostat_value(form.thermostat_setting);
    println!("New thermostat value: {}", form.thermostat_setting);
    HttpResponse::Ok().finish()
}

pub async fn get_thermostat(shared_data: web::Data<AccessSharedData>) -> HttpResponse {

    let thermostat = shared_data.get_thermostat_value();
    let thermostat = ThermostatData {
        thermostat_setting: thermostat,
    };
    let thermostat = serde_json::to_string(&thermostat).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(thermostat)
}
