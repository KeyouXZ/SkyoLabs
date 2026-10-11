use crate::se::core::Level;
use core::cell::RefCell;
use critical_section::Mutex;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct SensorData {
    pub mq7: Option<f32>,
    pub temperature_c: Option<f32>,
    pub humidity_pct: Option<f32>,
    pub temp_level: Option<Level>,
    pub hum_level: Option<Level>,
    pub gas_level: Option<Level>,
    pub level: Option<Level>,
}

static SENSOR_DATA: Mutex<RefCell<SensorData>> = Mutex::new(RefCell::new(SensorData {
    mq7: None,
    temperature_c: None,
    humidity_pct: None,
    temp_level: None,
    hum_level: None,
    gas_level: None,
    level: None,
}));

pub fn update_sensors(data: SensorData) {
    critical_section::with(|cs| {
        *SENSOR_DATA.borrow(cs).borrow_mut() = data;
    });
}

pub fn get_sensors() -> SensorData {
    critical_section::with(|cs| *SENSOR_DATA.borrow(cs).borrow())
}
