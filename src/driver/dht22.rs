use esp_hal::{
    delay::Delay,
    gpio::{Flex, InputConfig, OutputConfig},
};

pub struct Dht22<'d> {
    pin: Flex<'d>,
}

impl<'d> Dht22<'d> {
    pub fn new(mut pin: Flex<'d>) -> Self {
        pin.set_input_enable(true);
        pin.apply_input_config(&InputConfig::default());

        pin.set_output_enable(true);
        pin.apply_output_config(&OutputConfig::default());

        Self { pin }
    }

    /// Reads temperature (in °C) and humidity (in %) from the DHT22 sensor.
    pub fn read(&mut self, delay: &Delay) -> Result<(f32, f32), &'static str> {
        // 1. DATA LOW >= 1 ms (Drive low for 18 ms)
        self.pin.set_output_enable(true);
        self.pin.set_low();
        delay.delay_millis(18);

        // 2. DATA HIGH (Pull high for 20-40 µs, then release line by disabling output)
        self.pin.set_high();
        delay.delay_micros(30);
        self.pin.set_output_enable(false);

        // 3. Wait for DHT22 response (Low ~80 µs, then High ~80 µs)
        self.wait_for_level(false, 100, delay)?;
        self.wait_for_level(true, 100, delay)?;
        self.wait_for_level(false, 100, delay)?;

        // 4. Read 40 bits (5 bytes)
        let mut data = [0u8; 5];
        for byte in data.iter_mut() {
            for bit in (0..8).rev() {
                // Wait for the start of the bit pulse (Low -> High)
                self.wait_for_level(true, 100, delay)?;

                // Wait ~40 µs to sample whether it's still high (1) or dropped low (0)
                delay.delay_micros(40);

                if self.pin.is_high() {
                    *byte |= 1 << bit;
                }

                // Wait for the remainder of the high pulse to finish
                self.wait_for_level(false, 100, delay)?;
            }
        }

        // 5. Verify checksum
        let checksum = data[0]
            .wrapping_add(data[1])
            .wrapping_add(data[2])
            .wrapping_add(data[3]);
        if checksum != data[4] {
            return Err("DHT22 checksum mismatch");
        }

        // 6. Decode temperature and humidity
        let raw_humidity = ((data[0] as u16) << 8) | (data[1] as u16);
        let humidity = raw_humidity as f32 / 10.0;

        let raw_temp = (((data[2] & 0x7F) as u16) << 8) | (data[3] as u16);
        let mut temperature = raw_temp as f32 / 10.0;
        if (data[2] & 0x80) != 0 {
            temperature = -temperature;
        }

        Ok((temperature, humidity))
    }

    /// Helper with timeout protection to wait for a specific pin state
    fn wait_for_level(
        &self,
        target_high: bool,
        timeout_us: u32,
        delay: &Delay,
    ) -> Result<(), &'static str> {
        let mut elapsed = 0;
        while self.pin.is_high() != target_high {
            delay.delay_micros(1);
            elapsed += 1;
            if elapsed > timeout_us {
                return Err("DHT22 timeout waiting for pin level");
            }
        }
        Ok(())
    }
}
