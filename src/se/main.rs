use crate::{
    driver::{analog::AnalogReader, buzzer::Buzzer},
    se::{
        self,
        controller::BuzzerController,
        core::{CO_THRESHOLD, HUMIDITY_THRESHOLD, TEMPERATURE_THRESHOLD},
        display, utils,
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
    time::{Duration, Instant},
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

    // Create Header
    let header = display::create_header();

    // Main Loop 500ms delay
    loop {
        // TODO: Calibration MQ-7
        let mq7_val = mq7.read_raw() as f32;

        // DHT Value
        let (dht_temp, dht_hum) = match dht22.read(&Delay::default()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("{}", e);
                (50.0, 50.0)
            }
        };

        let temp_level = TEMPERATURE_THRESHOLD.classify(dht_temp);
        let hum_level = HUMIDITY_THRESHOLD.classify(dht_hum);
        let gas_level = CO_THRESHOLD.classify(mq7_val);
        let level = temp_level.max(hum_level).max(gas_level);

        log::info!(
            "temp: {:.2} hum: {:.2} gas: {:.2} level: {:?}",
            dht_temp,
            dht_hum,
            mq7_val,
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
            mq7_val,
            temp_level,
            hum_level,
            gas_level,
        );

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
