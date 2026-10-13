use crate::{
    driver::{analog::AnalogReader, buzzer::Buzzer},
    se::{
        self,
        controller::BuzzerController,
        core::{CO_THRESHOLD, HUMIDITY_THRESHOLD, TEMPERATURE_THRESHOLD},
        display,
        shared::{SensorData, update_sensors},
        utils::{self, set_delay},
    },
};
use core::{
    default::Default,
    result::Result::{Err, Ok},
};
use esp_hal::{
    analog::adc::{AdcChannel, RegisterAccess},
    delay::Delay,
    gpio::AnalogPin,
    i2c::master::Instance,
};

pub fn start<'a, ADC, MQ7, I2C>(pins: se::core::Pins<'a, ADC, MQ7, I2C>) -> !
where
    ADC: RegisterAccess,
    MQ7: AdcChannel + AnalogPin,
    I2C: Instance + 'a,
{
    // Main Program Setup
    // Setup MQ-7
    log::info!("Setup MQ-7");
    let mut mq7 = AnalogReader::new(pins.mq7_adc, pins.mq7);

    // Buzzer
    log::info!("Setup Buzzer");
    let buzzer = Buzzer::new(pins.buzzer);

    // Buzzer Controller
    let mut buzzer_controller = BuzzerController::new(buzzer);

    // Setup DHT22
    log::info!("Setup DHT22");
    let mut dht22 = utils::setup_dht22(pins.dht22);

    // Setup display SSD1306
    log::info!("Setup SSD1306");
    let mut ssd1306 =
        match utils::setup_ssd1306(pins.ssd1306_i2c, pins.ssd1306_sda, pins.ssd1306_scl) {
            Ok(x) => x,
            Err(e) => {
                panic!("{}", e);
            }
        };

    // Initialize SSD1306
    log::info!("Initialize SSD1306");
    if let Err(e) = ssd1306.init() {
        panic!("{}", e);
    }

    // Calibrating
    log::info!("Calibrating MQ-7 R0... Ensure sensor is in fresh air!");
    let r0 = match mq7.calibrate_r0(se::core::VCC, se::core::RL_OHMS, 50) {
        Some(calibrated_r0) => {
            log::info!("MQ-7 Calibrated R0: {:.1} Ohms", calibrated_r0);
            calibrated_r0
        }
        None => {
            log::warn!("Calibration failed, falling back to default R0 = 10000.0");
            10_000.0
        }
    };

    // Create Header
    let header = display::create_header();

    // Main Loop 500ms delay
    loop {
        // DHT Value
        let Ok((dht_temp, dht_hum)) = dht22.read(&Delay::default()) else {
            log::error!("Failed to read DHT22");
            set_delay(500);

            continue;
        };

        // Calculate Rs and CO PPM using calibrated R0
        let Some(rs) = mq7.read_rs(se::core::VCC, se::core::RL_OHMS) else {
            log::warn!("MQ-7 voltage out of range");
            set_delay(500);
            continue;
        };

        let Some(ppm) = utils::co_ppm(rs, r0) else {
            log::warn!("Invalid MQ-7 PPM calculation");
            set_delay(500);
            continue;
        };

        let temp_level = TEMPERATURE_THRESHOLD.classify(dht_temp);
        let hum_level = HUMIDITY_THRESHOLD.classify(dht_hum);
        let gas_level = CO_THRESHOLD.classify(ppm);
        let level = temp_level.max(hum_level).max(gas_level);

        update_sensors(SensorData {
            mq7: Some(ppm),
            temperature_c: Some(dht_temp),
            humidity_pct: Some(dht_hum),
            temp_level: Some(temp_level),
            hum_level: Some(hum_level),
            gas_level: Some(gas_level),
            level: Some(level),
        });

        log::info!(
            "temp: {:.2} hum: {:.2} gas: {:.2} level: {:?}",
            dht_temp,
            dht_hum,
            ppm,
            level
        );

        // Burst alarm based on level
        buzzer_controller.update(level);

        // Display the info
        display::display_info(
            &mut ssd1306,
            header,
            dht_temp,
            dht_hum,
            ppm,
            temp_level,
            hum_level,
            gas_level,
        );

        set_delay(500);
    }
}
