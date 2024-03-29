extern crate temp_data;

pub mod configuration;
pub mod control_thermostat;
pub mod pi_relay;
pub mod read_temp;
pub mod routes;
pub mod send_temp;
pub mod shared_data;
pub mod telemetry;

use crate::routes::get_thermostat_data::get_thermostat_data;
use crate::routes::health_check::health_check;
use crate::routes::temperature::get_temperature;
use crate::routes::thermostat::{get_thermostat, set_thermostat};
use crate::shared_data::AccessSharedData;

use actix_web::dev::Server;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// This function sets up the Actix web endpoints, gets each handler access to the shared data struct, and runs the server
// process.
pub fn run(listener: TcpListener, sd: &AccessSharedData) -> Result<Server, std::io::Error> {
    let common_data = web::Data::new(sd.clone());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(common_data.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/set_thermostat", web::post().to(set_thermostat))
            .route("/get_thermostat", web::get().to(get_thermostat))
            .route("/get_temperature", web::get().to(get_temperature))
            .route("/get_thermostat_data", web::get().to(get_thermostat_data))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
