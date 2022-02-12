use crate::shared_data::AccessSharedData;
use std::error::Error;
use time::OffsetDateTime;

// 2 minutes - minimum time before the thermostat can change state
const TIME_BUFFER: i64 = 120;

#[tracing::instrument(name = "seeing if the thermostat has to be set", skip(sd))]
fn control_thermostat(sd: &AccessSharedData) -> Result<(), Box<dyn Error>> {
    let temp_now = sd.get_current_temp();
    let thermostat_now = sd.get_thermostat_value();

    let now = OffsetDateTime::now_utc();
    tracing::debug!("now: {:?}", now);

    if now - sd.get_thermostat_change_datetime() > time::Duration::seconds(TIME_BUFFER) {
        if temp_now > thermostat_now as f32 && sd.is_thermostat_on() {
            tracing::debug!(
                "turning thermostat off: temp_now: {}, thermostat_now: {}, thermostat_on: {}",
                temp_now,
                thermostat_now,
                sd.is_thermostat_on()
            );
            // need to turn the thermostat to false
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

#[tracing::instrument(name = "running the loop to monitor thermostat setting", skip(sd))]
pub async fn run_control_thermostat(sd: &AccessSharedData) -> Result<(), Box<dyn Error>> {
    loop {
        control_thermostat(sd).unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        if !sd.get_continue_read_temp() {
            break;
        }
    }
    Ok(())
}
