pub mod configuration;
pub mod read_temp;
pub mod routes;
pub mod send_temp;
pub mod shared_data;

use crate::routes::health_check::health_check;
use crate::routes::temperature::get_temperature;
use crate::routes::thermostat::{get_thermostat, set_thermostat};
use crate::shared_data::AccessSharedData;

use actix_web::dev::Server;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use std::net::TcpListener;
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize)]
struct ThermostatData {
    thermostat_setting: usize,
}

pub fn run(listener: TcpListener, sd: &AccessSharedData) -> Result<Server, std::io::Error> {
    let common_data = web::Data::new(Arc::clone(&sd.sd));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(common_data.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/set_thermostat", web::post().to(set_thermostat))
            .route("/get_thermostat", web::get().to(get_thermostat))
            .route("/get_temperature", web::get().to(get_temperature))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
