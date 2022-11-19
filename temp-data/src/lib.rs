use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct TempData {
    pub record_date: String,
    pub thermostat_on: bool,
    pub temperature: f32,
    pub thermostat_value: u16,
}
