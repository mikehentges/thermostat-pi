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
    // Make sure our relay is set to off on app start
    thermostat_pi::pi_relay::change_relay(false).expect("unable to initialize relay");

    // Set up our telemetry output. Most values here are from environment variables
    let subscriber = get_subscriber("thermostat-pi".into(), "info".into());
    init_subscriber(subscriber);

    // Read our configuration file.
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Initialize a struct that will be our "global" data, which allows safe access from every thread
    let common_data = SharedData {
        continue_background_tasks: true,
        current_temp: configuration.initial_thermostat_value as f32 + 5.0,
        thermostat_value: configuration.initial_thermostat_value,
        thermostat_on: false,
        thermostat_change_datetime: OffsetDateTime::UNIX_EPOCH,
    };

    // The wrapper around our shared data that gives it safe access across threads
    let sd = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };

    // We are cloning the pointer to our shared data, and sending it into a new thread that continuously
    // reads the temperature from our sensor, and updates the SharedData::current_temp value
    let sdc = sd.clone();
    let run_handle = spawn(async move {
        tracing::debug!("kicking off read the temp");
        match read_the_temperature(
            &sdc,
            configuration.push_lambda_url,
            configuration.poll_interval,
        )
        .await
        {
            Ok(_) => tracing::info!("read has ended"),
            _ => tracing::error!("read has returned an error"),
        }
    });

    // Create another clone of our pointer to shared data, and send it into a new thread that continuously
    // checks to see how the current temperature and current thermostat setting compare - and will
    // trigger turning on the relay for the furnace as needed.
    let sdc = sd.clone();
    let control_handle = spawn(async move {
        tracing::debug!("kicking off control_thermostat");
        match run_control_thermostat(&sdc, configuration.poll_interval).await {
            Ok(_) => tracing::info!("control_thermostat has ended"),
            _ => tracing::error!("control_thermostat has returned an error"),
        }
    });

    // Create another clone of our pointer to shared data, and sned it into a signal handler.
    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, automatically set running to false.
    let sdc = sd.clone();
    let sig = tokio::spawn(async move {
        tracing::debug!("starting sig handler");
        tokio::signal::ctrl_c().await.unwrap();
        tracing::debug!("In handler for interrupt");
        {
            sdc.set_continue_background_tasks(false);
        }
        tracing::debug!("common data mutex set");
    });

    // Lastly, initialize our web server and listen for incoming instructions
    let listener = TcpListener::bind(format!("192.168.1.58:{}", configuration.application_port))
        .expect("Failed to create listener");
    let server = run(listener, &sd.clone())?;

    // hang out here until all of our threads shut down
    tokio::select! {
        _ = sig => 0,
        _ = run_handle => 0,
        _ = control_handle => 0,
        _ = server => 0
    };

    // Application is now shutting down, send out our final values
    tracing::info!(
        "Final temperature is: {}\nFinal thermostat value is: {}",
        sd.get_current_temp(),
        sd.get_thermostat_value()
    );

    // Make sure we don't leave the output on as we leave
    thermostat_pi::pi_relay::change_relay(false).expect("unable to shut down relay");
    Ok(())
}
