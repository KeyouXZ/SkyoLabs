use esp_hal::{
    Blocking,
    analog::adc::{Adc, AdcChannel, AdcConfig, AdcPin, Attenuation, RegisterAccess},
    gpio::AnalogPin,
};

pub struct AnalogReader<'a, ADC, PIN> {
    adc: Adc<'a, ADC, Blocking>,
    pin: AdcPin<PIN, ADC>,
}

impl<'a, ADC, PIN> AnalogReader<'a, ADC, PIN>
where
    ADC: RegisterAccess + 'a,
    PIN: AdcChannel + AnalogPin,
{
    pub fn new(adc_instance: ADC, gpio: PIN) -> Self {
        let mut config = AdcConfig::new();
        let pin = config.enable_pin(gpio, Attenuation::_11dB);
        let adc = Adc::new(adc_instance, config);

        Self { adc, pin }
    }

    fn l_read_raw(&mut self) -> u16 {
        nb::block!(self.adc.read_oneshot(&mut self.pin)).unwrap_or(0)
    }

    /// Read raw ADC value
    pub fn read_raw(&mut self) -> u16 {
        self.l_read_raw()
    }

    /// Read sensor voltage scaled to 3.3V ADC range
    pub fn read_voltage(&mut self) -> f32 {
        let raw = self.l_read_raw();
        (raw as f32 / 4095.0) * 3.3
    }

    /// Measure Rs given a load resistance (typically 10,000 ohms) and supply voltage VCC
    pub fn read_rs(&mut self, vcc: f32, rl: f32) -> Option<f32> {
        let v_out = self.read_voltage();
        if v_out <= 0.05 || v_out >= (vcc - 0.05) {
            return None; // Out of bounds or open circuit
        }

        // Rs = ((Vcc / Vout) - 1) * Rl
        Some(((vcc / v_out) - 1.0) * rl)
    }

    /// Self-calibration routine: averages readings over `samples` in fresh air
    pub fn calibrate_r0(&mut self, vcc: f32, rl: f32, samples: usize) -> Option<f32> {
        let mut rs_sum = 0.0;
        let mut valid_samples = 0;

        for _ in 0..samples {
            if let Some(rs) = self.read_rs(vcc, rl) {
                rs_sum += rs;
                valid_samples += 1;
            }
            crate::se::utils::set_delay(100);
        }

        if valid_samples == 0 {
            return None;
        }

        let rs_avg = rs_sum / (valid_samples as f32);
        // Clean air ratio Rs/R0 is ~27.0 according to MQ-7 datasheet
        Some(rs_avg / 27.0)
    }
}
