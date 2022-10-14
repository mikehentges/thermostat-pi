use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct TempData {
    pub record_date: String,
    pub thermostat_on: String,
    pub temperature: String,
    pub thermostat_value: String,
}
