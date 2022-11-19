use crate::shared_data::AccessSharedData;
use reqwest;
use reqwest::Error;
use time::format_description::well_known::Rfc3339;
use time::macros::offset;
use time::OffsetDateTime;

extern crate temp_data;
use temp_data::TempData;

#[tracing::instrument(name = "sending temp data to AWS", skip(sd), fields())]
pub async fn store_temp_data(sd: &AccessSharedData, aws_url: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();

    // Get the current time, offset to my timezone
    let now = OffsetDateTime::now_utc().to_offset(offset!(-6));
    let now = now.format(&Rfc3339).unwrap();
    tracing::debug!("now: {}", now);

    let body = TempData {
        record_date: now,
        thermostat_on: sd.is_thermostat_on(),
        temperature: sd.current_temp(),
        thermostat_value: sd.thermostat_value(),
    };
    tracing::debug!("json of struct: {:?}", serde_json::to_string(&body));

    let response = client
        .post(&format!("{}", aws_url))
        .json(&body)
        .send()
        .await;

    match response {
        Ok(r) => {
            tracing::debug!("response: {:?}", r);
        }
        Err(e) => {
            tracing::error!("Error sending to {}, {}", aws_url, e);
        }
    }

    Ok(())
}
