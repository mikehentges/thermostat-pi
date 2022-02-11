use crate::shared_data::AccessSharedData;
use log::{debug, error, info};
use std::error::Error;
use time::OffsetDateTime;
//use time::format_description::well_known::Rfc3339;
//use std::time::Duration;

// 5 minutes - minimum time before the thermostat can change state
const TIME_BUFFER :i64 = 3;

fn control_thermostat(
    sd: &AccessSharedData,
) -> Result<(), Box<dyn Error>> {
    let temp_now = sd.get_current_temp();
    let thermostat_now = sd.get_thermostat_value();

    let now = OffsetDateTime::now_utc();
    debug!("now: {:?}", now);

    if now - sd.get_thermostat_change_datetime() > time::Duration::seconds(TIME_BUFFER) {
        if temp_now > thermostat_now as f32 && sd.is_thermostat_on() {
            debug!("turning thermostat off: temp_now: {}, thermostat_now: {}, thermostat_on: {}", temp_now, thermostat_now, sd.is_thermostat_on());
            // need to turn the thermostat to false
            sd.set_thermostat_on(false);
            sd.set_thermostat_change_datetime(now);
        } else if temp_now < thermostat_now as f32 && !sd.is_thermostat_on() {
            debug!("turning thermostat on: temp_now: {}, thermostat_now: {}, thermostat_on: {}", temp_now, thermostat_now, sd.is_thermostat_on());
            // need to turn the thermostat to true
            sd.set_thermostat_on(true);
            sd.set_thermostat_change_datetime(now);
        }
        else {
            // No change to thermostat needed
            info!("thermostat OK: temp_now: {}, thermostat_now: {}, thermostat_on: {}", temp_now, thermostat_now, sd.is_thermostat_on());
        }
    }
    Ok(())
}

pub async fn run_control_thermostat( sd: &AccessSharedData, ) -> Result<(), Box<dyn Error>> {
    loop {
        control_thermostat(sd).unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        if !sd.get_continue_read_temp() {
            break;
        }
    }
    Ok(())
}