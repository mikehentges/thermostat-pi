use log::{debug, error, info};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;
use thermostat_pi::configuration::get_configuration;
use thermostat_pi::read_temp::read_the_temperature;
use thermostat_pi::control_thermostat::run_control_thermostat;
use thermostat_pi::run;
use thermostat_pi::shared_data::{AccessSharedData, SharedData};
use tokio::spawn;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    //let rt = Runtime::new().unwrap();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let common_data = SharedData {
        continue_read_temp: true,
        current_temp: configuration.initial_thermostat_value as f32 + 5.0,
        thermostat_value: configuration.initial_thermostat_value,
        thermostat_on: false,
        thermostat_change_datetime: OffsetDateTime::UNIX_EPOCH,
    };
    let sd = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };

    let sdc = sd.clone();
    let run_handle = spawn(async move {
        debug!("kicking off read the temp");
        match read_the_temperature(&sdc, configuration.push_lambda_url).await {
            Ok(_) => info!("read has ended"),
            _ => error!("read has returned an error"),
        }
    });
    let sdc = sd.clone();
    let control_handle = spawn(async move {
        debug!("kicking off control_thermostat");
        match run_control_thermostat(&sdc).await {
            Ok(_) => info!("control_thermostat has ended"),
            _ => error!("control_thermostat has returned an error"),
        }
    });


    let sdc = sd.clone();

    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, automatically set running to false.
    let sig = tokio::spawn(async move {
        debug!("starting sig handler");
        tokio::signal::ctrl_c().await.unwrap();
        debug!("In handler for interrupt");
        {
            sdc.set_continue_read_temp(false);
        }
        debug!("common data mutex set");
    });

    let listener = TcpListener::bind(format!("192.168.1.33:{}", configuration.application_port))
        .expect("Failed to create listener");
    let server = run(listener, &sd.clone())?;
    tokio::select! {
        _ = sig => 0,
        _ = run_handle => 0,
        _ = control_handle => 0,
        _ = server => 0
    };
    info!(
        "Final temperature is: {}\nFinal thermostat value is: {}",
        sd.get_current_temp(),
        sd.get_thermostat_value()
    );
    Ok(())
}
