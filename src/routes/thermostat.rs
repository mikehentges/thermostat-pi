use crate::shared_data::SharedData;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use std::sync::{Arc, Mutex};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ThermostatData {
    thermostat_setting: usize,
}

pub async fn set_thermostat(
    form: web::Form<ThermostatData>,
    shared_data: web::Data<Arc<Mutex<SharedData>>>,
) -> HttpResponse {
    let mut shared_data = shared_data.lock().unwrap();
    shared_data.thermostat_value = form.thermostat_setting;
    println!("New thermostat value: {}", form.thermostat_setting);
    HttpResponse::Ok().finish()
}

pub async fn get_thermostat(shared_data: web::Data<Arc<Mutex<SharedData>>>) -> HttpResponse {
    let thermostat = shared_data.lock().unwrap().thermostat_value;
    let thermostat = ThermostatData {
        thermostat_setting: thermostat,
    };
    let thermostat = serde_json::to_string(&thermostat).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(thermostat)
}
