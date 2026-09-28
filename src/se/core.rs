use crate::driver::ssd1306::Ssd1306Size;
use esp_hal::{
    analog::adc::{AdcChannel, RegisterAccess},
    gpio::{AnalogPin, AnyPin},
    i2c::master::Instance,
};

/// SSD1306 Resolution
pub const SSD1306_SCREEN_SIZE: Ssd1306Size = Ssd1306Size::S128x64;

pub struct Pins<'a, ADC, MQ7, I2C>
where
    ADC: RegisterAccess,
    MQ7: AdcChannel + AnalogPin,
    I2C: Instance,
{
    /// MQ-7 Pin
    pub mq7: MQ7,
    /// MQ-7 ADC Pin
    pub mq7_adc: ADC,

    /// SSD1306 I2C Pin
    pub ssd1306_i2c: I2C,
    /// SSD1306 SDA Pin
    pub ssd1306_sda: AnyPin<'a>,
    /// SSD1306 SCL Pin
    pub ssd1306_scl: AnyPin<'a>,

    /// DHT22 Pin
    pub dht22: AnyPin<'a>,

    /// Buzzer Pin
    pub buzzer: AnyPin<'a>,
}

/// Struct for Threshold
pub struct Threshold {
    pub safe: (f32, f32),
    pub warning: (f32, f32),
    pub danger: (f32, f32),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Level {
    Safe,
    Warning,
    Danger,
}

impl Threshold {
    fn in_range(range: (f32, f32), value: f32) -> bool {
        value >= range.0 && value < range.1
    }

    pub fn classify(&self, value: f32) -> Level {
        if Self::in_range(self.safe, value) {
            Level::Safe
        } else if Self::in_range(self.warning, value) {
            Level::Warning
        } else {
            Level::Danger
        }
    }
}

/// Temperature Threshold
pub const TEMPERATURE_THRESHOLD: Threshold = Threshold {
    safe: (20.0, 35.0),
    warning: (15.0, 40.0),
    danger: (f32::NEG_INFINITY, f32::INFINITY),
};

/// Humidity Threshold
/// Carefull, if the Threshold::classify changed, then consider to read this again
pub const HUMIDITY_THRESHOLD: Threshold = Threshold {
    safe: (60.0, 70.0),
    warning: (55.0, 80.0),
    danger: (0.0, 100.0),
};

/// Need Calibration
pub const CO_THRESHOLD: Threshold = Threshold {
    safe: (0.0, 3500.0),
    warning: (3500.0, 4500.0),
    danger: (4500.0, f32::INFINITY),
};

// /// PPM Calibration first
// pub const CO_THRESHOLD: Threshold = Threshold {
//     safe: (0.0, 50.0),
//     warning: (51.0, 100.0),
//     danger: (101.0, f32::INFINITY),
// };
