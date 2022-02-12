use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use thermostat_pi::configuration::get_configuration;
use thermostat_pi::control_thermostat::run_control_thermostat;
use thermostat_pi::read_temp::read_the_temperature;
use thermostat_pi::run;
use thermostat_pi::shared_data::{AccessSharedData, SharedData};
use thermostat_pi::telemetry::{get_subscriber, init_subscriber};
use time::OffsetDateTime;
use tokio::spawn;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("thermostat-pi".into(), "info".into());
    init_subscriber(subscriber);

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
        tracing::debug!("kicking off read the temp");
        match read_the_temperature(&sdc, configuration.push_lambda_url).await {
            Ok(_) => tracing::info!("read has ended"),
            _ => tracing::error!("read has returned an error"),
        }
    });
    let sdc = sd.clone();
    let control_handle = spawn(async move {
        tracing::debug!("kicking off control_thermostat");
        match run_control_thermostat(&sdc).await {
            Ok(_) => tracing::info!("control_thermostat has ended"),
            _ => tracing::error!("control_thermostat has returned an error"),
        }
    });

    let sdc = sd.clone();

    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, automatically set running to false.
    let sig = tokio::spawn(async move {
        tracing::debug!("starting sig handler");
        tokio::signal::ctrl_c().await.unwrap();
        tracing::debug!("In handler for interrupt");
        {
            sdc.set_continue_read_temp(false);
        }
        tracing::debug!("common data mutex set");
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
    tracing::info!(
        "Final temperature is: {}\nFinal thermostat value is: {}",
        sd.get_current_temp(),
        sd.get_thermostat_value()
    );
    Ok(())
}
