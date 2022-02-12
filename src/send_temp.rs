use crate::shared_data::AccessSharedData;
use reqwest;
use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;
use time::format_description::well_known::Rfc3339;
use time::macros::offset;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
struct TempData {
    pub record_date: String,
    pub thermostat_on: String,
    pub temperature: String,
    pub thermostat_value: String,
}
#[tracing::instrument(name = "sending temp data to AWS", skip(sd), fields())]
pub async fn store_temp_data(sd: &AccessSharedData, aws_url: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let now = OffsetDateTime::now_utc().to_offset(offset!(-6));
    let now = now.format(&Rfc3339).unwrap();
    tracing::debug!("now: {}", now);

    let body = TempData {
        record_date: now,
        thermostat_on: sd.is_thermostat_on().to_string(),
        temperature: sd.get_current_temp().to_string(),
        thermostat_value: sd.get_thermostat_value().to_string(),
    };
    tracing::debug!("json of struct: {:?}", serde_json::to_string(&body));

    let response = client
        .post(&format!("{}/push_temp", aws_url))
        .json(&body)
        .send()
        .await?;
    tracing::debug!("response: {:?}", response);

    Ok(())
}
