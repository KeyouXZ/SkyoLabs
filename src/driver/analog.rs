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

    /// Read sensor voltage
    pub fn read_voltage(&mut self) -> f32 {
        let raw = self.l_read_raw();

        (raw as f32 / 4095.0) * 3.3
    }

    /// Read raw ADC value
    pub fn read_raw(&mut self) -> u16 {
        self.l_read_raw()
    }
}
