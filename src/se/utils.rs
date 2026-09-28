use crate::{
    driver::{dht22::Dht22, ssd1306::Ssd1306},
    se::core::SSD1306_SCREEN_SIZE,
};
use core::default::Default;
use esp_hal::{
    gpio::{AnyPin, Flex},
    i2c::master::{Config, I2c, Instance},
    time::Rate,
};

pub fn setup_dht22<'a>(pin: AnyPin<'a>) -> Dht22<'a> {
    let dht22_pin = Flex::new(pin);
    Dht22::new(dht22_pin)
}

pub fn setup_ssd1306<'a, I2C>(
    i2c_device: I2C,
    sda: AnyPin<'a>,
    scl: AnyPin<'a>,
) -> anyhow::Result<Ssd1306<'a>>
where
    I2C: Instance + 'a,
{
    let i2c = I2c::new(
        i2c_device,
        Config::default().with_frequency(Rate::from_hz(400)),
    )?
    .with_sda(sda)
    .with_scl(scl);

    let mut display = Ssd1306::new(i2c, SSD1306_SCREEN_SIZE);
    display.init()?;

    Ok(display)
}
