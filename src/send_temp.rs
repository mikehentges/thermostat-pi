use crate::shared_data::AccessSharedData;
//use chrono::{DateTime, Utc};
use log::{debug, error, info};
use reqwest;
use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use time::macros::offset;

#[derive(Debug, Serialize, Deserialize)]
struct TempData {
    pub record_date: String,
    pub thermostat_on: String,
    pub temperature: String,
    pub thermostat_value: String,
}

pub async fn store_temp_data(sd: &AccessSharedData, aws_url: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let now = OffsetDateTime::now_utc().to_offset(offset!(-6));
    let now = now.format(&Rfc3339).unwrap();
    debug!("now: {}", now);

    let body = TempData {
        record_date: now,
        thermostat_on: sd.is_thermostat_on().to_string(),
        temperature: sd.get_current_temp().to_string(),
        thermostat_value: sd.get_thermostat_value().to_string(),
    };
    debug!("json of struct: {:?}", serde_json::to_string(&body));

    let response = client
        .post(&format!("{}/push_temp", aws_url))
        .json(&body)
        .send()
        .await?;
    debug!("response: {:?}", response);

    Ok(())
}
