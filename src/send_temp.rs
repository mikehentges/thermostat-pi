use crate::shared_data::AccessSharedData;
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use reqwest;
use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
struct TempData {
    pub record_date: String,
    pub thermostat_on: String,
    pub temperature: String,
    pub thermostat_value: String,
}

pub async fn store_temp_data(sd: &AccessSharedData, aws_url: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339();
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
