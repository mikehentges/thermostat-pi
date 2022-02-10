use std::sync::{Arc, Mutex};

pub struct SharedData {
    pub continue_read_temp: bool,
    pub current_temp: f32,
    pub thermostat_value: usize,
    pub thermostat_on: bool,
}
pub type Sd = Arc<Mutex<SharedData>>;

pub struct AccessSharedData {
    pub sd: Sd,
}

impl Clone for AccessSharedData {
    fn clone(&self) -> Self {
        AccessSharedData {
            sd: Arc::clone(&self.sd),
        }
    }
}

impl AccessSharedData {
    pub fn get_continue_read_temp(&self) -> bool {
        let lock = self.sd.lock().unwrap();
        lock.continue_read_temp
    }
    pub fn set_continue_read_temp(&self, new_val: bool) {
        let mut lock = self.sd.lock().unwrap();
        lock.continue_read_temp = new_val;
    }
    pub fn get_current_temp(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.current_temp
    }
    pub fn set_current_temp(&self, new_val: f32 ) {
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
}
