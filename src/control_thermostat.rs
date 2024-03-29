use crate::pi_relay::change_relay;
use crate::shared_data::AccessSharedData;
use std::error::Error;
use time::OffsetDateTime;

// 2 minutes - minimum time before the thermostat can change state
const TIME_BUFFER: i64 = 120;

#[derive(serde::Deserialize, serde::Serialize)]
struct ThermostatData {
    thermostat_setting: usize,
}

#[tracing::instrument(name = "seeing if the thermostat has to be set", skip(sd))]
fn control_thermostat(sd: &AccessSharedData) -> Result<(), Box<dyn Error>> {
    let temp_now = sd.current_temp();
    let thermostat_now = sd.thermostat_value();

    let now = OffsetDateTime::now_utc();
    tracing::debug!("now: {:?}", now);

    if now - sd.thermostat_change_datetime() > time::Duration::seconds(TIME_BUFFER) {
        if temp_now > thermostat_now as f32 && sd.is_thermostat_on() {
            tracing::debug!(
                "turning thermostat off: temp_now: {}, thermostat_now: {}, thermostat_on: {}",
                temp_now,
                thermostat_now,
                sd.is_thermostat_on()
            );
            // need to turn the thermostat to false
            change_relay(false)?;
            sd.set_thermostat_on(false);
            sd.set_thermostat_change_datetime(now);
        } else if temp_now < thermostat_now as f32 && !sd.is_thermostat_on() {
            tracing::debug!(
                "turning thermostat on: temp_now: {}, thermostat_now: {}, thermostat_on: {}",
                temp_now,
                thermostat_now,
                sd.is_thermostat_on()
            );
            // need to turn the thermostat to true
            change_relay(true)?;
            sd.set_thermostat_on(true);
            sd.set_thermostat_change_datetime(now);
        } else {
            // No change to thermostat needed
            tracing::info!(
                "thermostat OK: temp_now: {}, thermostat_now: {}, thermostat_on: {}",
                temp_now,
                thermostat_now,
                sd.is_thermostat_on()
            );
        }
    }
    Ok(())
}

pub async fn run_control_thermostat(
    sd: &AccessSharedData,
    poll_interval: usize,
) -> Result<(), Box<dyn Error>> {
    loop {
        control_thermostat(sd).unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(poll_interval as u64)).await;
        if !sd.continue_background_tasks() {
            break;
        }
    }
    Ok(())
}
