use crate::SharedData;
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use reqwest;
use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
struct TempData {
    pub record_date: String,
    pub thermostat_on: String,
    pub temperature: String,
    pub thermostat_value: String,
}
//const AWS_URL: &str = "https://tktt1n58z8.execute-api.us-east-2.amazonaws.com";
// pub struct shared_data {
//     continue_read_temp: bool,
//     current_temp: f32,
//     thermostat_value: usize,
//     thermostat_on: bool,
// }

pub async fn store_temp_data(
    common_data: &Arc<Mutex<SharedData>>,
    aws_url: &str,
) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339();
    debug!("now: {}", now);

    let common_data = &*common_data.lock().unwrap();

    let body = TempData {
        record_date: now,
        thermostat_on: common_data.thermostat_on.to_string(),
        temperature: common_data.current_temp.to_string(),
        thermostat_value: common_data.thermostat_value.to_string(),
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
