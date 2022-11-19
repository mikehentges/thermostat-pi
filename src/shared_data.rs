use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

// A struct to hold the values that will be shared across all threads in the application
pub struct SharedData {
    continue_background_tasks: bool,
    current_temp: f32,
    thermostat_value: u16,
    thermostat_on: bool,
    thermostat_change_datetime: OffsetDateTime,
}

impl SharedData {
    pub fn new(
        continue_background_tasks: bool,
        current_temp: f32,
        thermostat_value: u16,
        thermostat_on: bool,
        thermostat_change_datetime: OffsetDateTime,
    ) -> SharedData {
        SharedData {
            continue_background_tasks,
            current_temp,
            thermostat_value,
            thermostat_on,
            thermostat_change_datetime,
        }
    }
}

// The struct that will be used to manage access to the shared data struct.
pub struct AccessSharedData {
    pub sd: Arc<Mutex<SharedData>>,
}

// Clone here just makes a copy of the Arc pointer - not  the entire class of data
// All clones point to the same internal data
impl Clone for AccessSharedData {
    fn clone(&self) -> Self {
        AccessSharedData {
            sd: Arc::clone(&self.sd),
        }
    }
}

// Getters/Setters for access to the shared data. Everything is wrapped in a MutexGuard to
// ensure thread safety for every access point.
impl AccessSharedData {
    pub fn continue_background_tasks(&self) -> bool {
        let lock = self.sd.lock().unwrap();
        lock.continue_background_tasks
    }
    pub fn set_continue_background_tasks(&self, new_val: bool) {
        let mut lock = self.sd.lock().unwrap();
        lock.continue_background_tasks = new_val;
    }
    pub fn current_temp(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.current_temp
    }
    pub fn set_current_temp(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.current_temp = new_val;
    }
    pub fn thermostat_value(&self) -> u16 {
        let lock = self.sd.lock().unwrap();
        lock.thermostat_value
    }
    pub fn set_thermostat_value(&self, new_val: u16) {
        let mut lock = self.sd.lock().unwrap();
        lock.thermostat_value = new_val;
    }
    pub fn is_thermostat_on(&self) -> bool {
        let lock = self.sd.lock().unwrap();
        lock.thermostat_on
    }
    pub fn set_thermostat_on(&self, new_val: bool) {
        let mut lock = self.sd.lock().unwrap();
        lock.thermostat_on = new_val;
    }
    pub fn thermostat_change_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.thermostat_change_datetime
    }
    pub fn set_thermostat_change_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.thermostat_change_datetime = dt;
    }
}
