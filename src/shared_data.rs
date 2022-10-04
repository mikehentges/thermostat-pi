use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

// A struct to hold the values that will be shared across all threads in the application
pub struct SharedData {
    pub continue_background_tasks: bool,
    pub current_temp: f32,
    pub thermostat_value: usize,
    pub thermostat_on: bool,
    pub thermostat_change_datetime: OffsetDateTime,
}

pub type Sd = Arc<Mutex<SharedData>>;

// The struct that will be used to manage access to the shared data struct.
pub struct AccessSharedData {
    pub sd: Sd,
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
    pub fn get_continue_background_tasks(&self) -> bool {
        let lock = self.sd.lock().unwrap();
        lock.continue_background_tasks
    }
    pub fn set_continue_background_tasks(&self, new_val: bool) {
        let mut lock = self.sd.lock().unwrap();
        lock.continue_background_tasks = new_val;
    }
    pub fn get_current_temp(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.current_temp
    }
    pub fn set_current_temp(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.current_temp = new_val;
    }
    pub fn get_thermostat_value(&self) -> usize {
        let lock = self.sd.lock().unwrap();
        lock.thermostat_value
    }
    pub fn set_thermostat_value(&self, new_val: usize) {
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
    pub fn get_thermostat_change_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.thermostat_change_datetime
    }
    pub fn set_thermostat_change_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.thermostat_change_datetime = dt;
    }
}
